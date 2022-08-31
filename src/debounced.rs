use crate::table::{DIR_CCW, DIR_CW, STATE_TABLE_FULL_STEPS, STATE_TABLE_HALF_STEPS};
use crate::Direction;
use crate::RotaryEncoder;
use embedded_hal::digital::v2::InputPin;

/// Debounce mode
pub struct DebouncedMode {
    pub table_full_state: u8,
    pub table_half_state: u8,
    pub last_half_millis: u64,
    pub last_full_millis: u64,
}
impl<DT, CLK> RotaryEncoder<DebouncedMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Update the state machine of the RotaryEncoder. This should be called ideally from an interrupt vector
    /// when either the DT or CLK pins state changes. This function will update the RotaryEncoder's Direction
    pub fn update(&mut self, millis: u64) {
        let dt_state = self.pin_dt.is_high().unwrap_or_default() as u8;
        let clk_state = self.pin_clk.is_high().unwrap_or_default() as u8;

        let pin_state = (dt_state << 1) | clk_state;

        self.mode.table_full_state =
            STATE_TABLE_FULL_STEPS[self.mode.table_full_state as usize & 0x0F][pin_state as usize];

        let full_dir = self.mode.table_full_state & 0x30;

        let direction = match full_dir {
            DIR_CW => Direction::Clockwise,
            DIR_CCW => Direction::Anticlockwise,
            _ => Direction::None,
        };

        // The half state will fire before the whole state

        // Simply take the full direction
        if direction != Direction::None && (millis - self.mode.last_half_millis) > 80 {
            // This handles faster spinning
            self.mode.last_full_millis = millis;
            self.direction = direction;
            return;
        }

        // Otherwise use the half direction
        // This handles slower spinning
        self.mode.table_half_state =
            STATE_TABLE_HALF_STEPS[self.mode.table_half_state as usize & 0x0F][pin_state as usize];
        let half_dir = self.mode.table_half_state & 0x30;

        let direction = match half_dir {
            DIR_CW => Direction::Clockwise,
            DIR_CCW => Direction::Anticlockwise,
            _ => Direction::None,
        };
        if direction != Direction::None
            && (millis - self.mode.last_full_millis) > 80
            && (millis - self.mode.last_half_millis) > 50
        {
            self.mode.last_half_millis = millis;
            self.direction = direction;
        } else {
            self.direction = Direction::None;
        }
    }
}

impl<DT, CLK, MODE> RotaryEncoder<MODE, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Configure RotaryEncoder to use the debounce API
    pub fn into_debounced_mode(self) -> RotaryEncoder<DebouncedMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: DebouncedMode {
                table_full_state: 0,
                table_half_state: 0,
                last_half_millis: 0,
                last_full_millis: 0,
            },
            direction: Direction::None,
        }
    }
}
