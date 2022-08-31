use crate::table::{DIR_CCW, DIR_CW, STATE_TABLE_HALF_STEPS};
use crate::Direction;
use crate::RotaryEncoder;
use embedded_hal::digital::v2::InputPin;

pub static mut TABLE_STATE: u8 = 0;
pub static mut FLOW: u8 = 0;
pub static mut PIN_STATE: f32 = 0.0;

/// Debounce mode
pub struct DebouncedMode {
    pub table_state: u8,
    pub last_update_millis: u64,
    pub factor: f32,
}
impl<DT, CLK> RotaryEncoder<DebouncedMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    pub fn decay(&mut self, dt: f32) {
        // In a quater of a second the value of factor should go to 0.
        let dec = dt * 4.0;
        let mut factor = self.mode.factor;
        if factor >= 0.0 {
            factor -= dt;
            if factor < 0.0 {
                factor = 0.0;
            }
        }
        self.mode.factor = factor;

        unsafe {
            PIN_STATE = self.mode.factor;
        }
    }

    /// Update the state machine of the RotaryEncoder. This should be called ideally from an interrupt vector
    /// when either the DT or CLK pins state changes. This function will update the RotaryEncoder's Direction
    pub fn update(&mut self, millis: u64) {
        let dt_state = self.pin_dt.is_high().unwrap_or_default() as u8;
        let clk_state = self.pin_clk.is_high().unwrap_or_default() as u8;
        let pin_state = dt_state << 1 | clk_state;

        unsafe {
            TABLE_STATE = self.mode.table_state;
            FLOW = self.mode.table_state & 0x0F;
        }

        self.mode.table_state =
            STATE_TABLE_HALF_STEPS[self.mode.table_state as usize & 0x0F][pin_state as usize];
        let half_dir = self.mode.table_state & 0x30;

        let direction = match half_dir {
            DIR_CW => Direction::Clockwise,
            DIR_CCW => Direction::Anticlockwise,
            _ => Direction::None,
        };
        if direction != Direction::None
            && (millis - self.mode.last_update_millis)
                > (55 - (25 as f32 * self.mode.factor) as u64)
        {
            let mut factor = self.mode.factor;
            if factor < 1.0 {
                factor += 0.25;
                if factor > 1.0 {
                    factor = 1.0;
                }
            }
            self.mode.factor = factor;
            self.mode.table_state = 0;
            self.mode.last_update_millis = millis;
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
                table_state: 0,
                last_update_millis: 0,
                factor: 0.0,
            },
            direction: Direction::None,
        }
    }
}
