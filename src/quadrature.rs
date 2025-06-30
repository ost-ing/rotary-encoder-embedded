use embedded_hal::digital::InputPin;

use crate::{Direction, RotaryEncoder};

/// Quadrature Lookup Table
/// Index = (prev_state << 2) | curr_state
/// Value = +1 for CW, -1 for CCW, 0 for no movement/invalid
const QUAD_TABLE: [i8; 16] = [
    0,  // 00 -> 00
    1,  // 00 -> 01  = CW
    -1, // 00 -> 10  = CCW
    0,  // 00 -> 11  = invalid (skipped state)
    -1, // 01 -> 00  = CCW
    0,  // 01 -> 01
    0,  // 01 -> 10  = invalid
    1,  // 01 -> 11  = CW
    1,  // 10 -> 00  = CW
    0,  // 10 -> 01  = invalid
    0,  // 10 -> 10
    -1, // 10 -> 11  = CCW
    0,  // 11 -> 00  = invalid
    -1, // 11 -> 01  = CCW
    1,  // 11 -> 10  = CW
    0,  // 11 -> 11
];

impl<DT, CLK> RotaryEncoder<QuadratureTableMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Updates the `RotaryEncoder`, updating the `direction` property
    pub fn update(&mut self) -> Direction {
        self.mode.update(
            self.pin_dt.is_high().unwrap_or_default(),
            self.pin_clk.is_high().unwrap_or_default(),
        )
    }
}

impl<LOGIC, DT, CLK> RotaryEncoder<LOGIC, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Configure `RotaryEncoder` to use the quadrature table mode
    pub fn into_quadrature_table_mode(
        self,
        threshold: u8,
    ) -> RotaryEncoder<QuadratureTableMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: QuadratureTableMode::new(threshold),
        }
    }
}

impl Default for QuadratureTableMode {
    fn default() -> Self {
        Self::new(1)
    }
}

/// Quadrature Table Encoder Mode
/// This mode is suitable for indentless encoders
pub struct QuadratureTableMode {
    prev_state: u8, // lower two bits only
    threshold: u8,  // how many “deltas” before we report a step
    count: i8,      // running sum of +1/–1 deltas
}

impl QuadratureTableMode {
    /// Initializes Quadrature table encoder
    /// `threshold` - the number of events before a Direction is yielded. By default this value is 1 for the most sensitivity.
    pub fn new(threshold: u8) -> Self {
        Self {
            prev_state: 0,
            count: 0,
            threshold,
        }
    }

    /// Call this on every A/B change (or in a tight loop)
    /// dt = data pin, clk = clock pin levels (0 or 1)
    pub fn update(&mut self, dt: bool, clk: bool) -> Direction {
        let curr = (dt as u8) | ((clk as u8) << 1);
        let idx = ((self.prev_state << 2) | curr) as usize;
        let delta = QUAD_TABLE[idx];
        self.prev_state = curr;
        self.count += delta;
        if self.count.abs() as u8 >= self.threshold {
            let dir = if self.count > 0 {
                Direction::Clockwise
            } else {
                Direction::Anticlockwise
            };
            self.count = 0; // reset and only report once
            return dir;
        }
        Direction::None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;

    /// Test‑only helper; collects the result of each update.
    fn drive_sequence(mode: &mut QuadratureTableMode, seq: &[(bool, bool)]) -> Vec<Direction> {
        seq.iter().map(|&(dt, clk)| mode.update(dt, clk)).collect()
    }

    #[test]
    fn single_cw_step_threshold_1() {
        let mut mode = QuadratureTableMode::new(1);
        // 00 -> 01 (+1) => immediately CW
        assert_eq!(mode.update(true, false), Direction::Clockwise);
    }

    #[test]
    fn single_ccw_step_threshold_1() {
        let mut mode = QuadratureTableMode::new(1);
        // 00 -> 10 (–1) => immediately CCW
        assert_eq!(mode.update(false, true), Direction::Anticlockwise);
    }

    #[test]
    fn aggregation_threshold_2_requires_two_valid_pulses() {
        // threshold = 2: need two *valid* deltas before firing
        let mut mode = QuadratureTableMode::new(2);

        // First valid CW pulse: 00->01 = +1, count=1 < 2 => None
        assert_eq!(mode.update(true, false), Direction::None);

        // Next valid CW pulse: 01->11 = +1, count=2 >= 2 => CW
        assert_eq!(mode.update(true, true), Direction::Clockwise);

        // Counter reset: another pulse 11->10 = +1 gives None
        assert_eq!(mode.update(false, true), Direction::None);
    }

    #[test]
    fn no_movement_on_constant_state() {
        let mut mode = QuadratureTableMode::new(1);
        // Stay in 00 the whole time
        for _ in 0..5 {
            assert_eq!(mode.update(false, false), Direction::None);
        }
    }

    #[test]
    fn invalid_transition_skipped_state() {
        let mut mode = QuadratureTableMode::new(1);
        // 00 -> 11 is invalid (table[0b0011] == 0)
        assert_eq!(mode.update(true, true), Direction::None);
        // And back 11 -> 00 is also table[0b1100] == 0
        assert_eq!(mode.update(false, false), Direction::None);
    }

    #[test]
    fn full_cw_cycle_threshold_1() {
        let mut mode = QuadratureTableMode::new(1);
        // A full 4‑step CW quadrature cycle: 00→01→11→10→00
        let seq = [
            (false, false), // 00→00 = 0
            (true, false),  // 00→01 = +1 → CW
            (true, true),   // 01→11 = +1 → CW
            (false, true),  // 11→10 = +1 → CW
            (false, false), // 10→00 = +1 → CW
        ];
        let results = drive_sequence(&mut mode, &seq);
        assert_eq!(
            results,
            vec![
                Direction::None,
                Direction::Clockwise,
                Direction::Clockwise,
                Direction::Clockwise,
                Direction::Clockwise,
            ]
        );
    }
}
