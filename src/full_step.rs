use crate::table::{full::STATE_TABLE_FULL_STEPS, DIR_CCW, DIR_CW};
use crate::Direction;
use crate::RotaryEncoder;
use embedded_hal::digital::v2::InputPin;

/// FullStep Mode
/// Uses a table state machine for transitions and only considers "full-step" transitions, i.e. "half-steps" are ignored.
/// Overall this method works well but can occassionally yield some errors depending on the rotary encoder,
/// e.g. turning the knob may not yield a state change
pub struct FullStepMode {
    table_state: u8,
}

impl<DT, CLK> RotaryEncoder<FullStepMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Updates the `RotaryEncoder`, updating the `direction` property
    pub fn update(&mut self) {
        let dt_state = self.pin_dt.is_high().unwrap_or_default() as u8;
        let clk_state = self.pin_clk.is_high().unwrap_or_default() as u8;
        let pin_state = dt_state << 1 | clk_state;
        self.mode.table_state =
            STATE_TABLE_FULL_STEPS[self.mode.table_state as usize & 0x0F][pin_state as usize];
        let dir = self.mode.table_state & 0x30;
        self.direction = match dir {
            DIR_CW => Direction::Clockwise,
            DIR_CCW => Direction::Anticlockwise,
            _ => Direction::None,
        };
    }
}

impl<DT, CLK, MODE> RotaryEncoder<MODE, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Configure `RotaryEncoder` to use the full step API
    pub fn into_fullstep_mode(self) -> RotaryEncoder<FullStepMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: FullStepMode { table_state: 0 },
            direction: Direction::None,
        }
    }
}
