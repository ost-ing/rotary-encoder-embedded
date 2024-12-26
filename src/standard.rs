use embedded_hal::digital::InputPin;

use crate::Direction;
use crate::RotaryEncoder;

/// StandardMode
/// This mode is best used when polled at ~900Hz.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StandardMode {
    /// The pin state
    pin_state: [u8; 2],
}

// For debouncing of pins, use 0x0f (b00001111) and 0x0c (b00001100) etc.
const PIN_MASK: u8 = 0x03;
const PIN_EDGE: u8 = 0x02;

impl<DT, CLK> RotaryEncoder<StandardMode, DT, CLK>
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

impl StandardMode {
    /// Initialises the StandardMode
    pub fn new() -> Self {
        Self {
            pin_state: [0xFF, 2],
        }
    }

    /// Update to determine the direction
    pub fn update(&mut self, dt_value: bool, clk_value: bool) -> Direction {
        self.pin_state[0] = (self.pin_state[0] << 1) | dt_value as u8;
        self.pin_state[1] = (self.pin_state[1] << 1) | clk_value as u8;

        let a = self.pin_state[0] & PIN_MASK;
        let b = self.pin_state[1] & PIN_MASK;

        let mut dir: Direction = Direction::None;

        if a == PIN_EDGE && b == 0x00 {
            dir = Direction::Anticlockwise;
        } else if b == PIN_EDGE && a == 0x00 {
            dir = Direction::Clockwise;
        }

        dir
    }
}

impl<LOGIC, DT, CLK> RotaryEncoder<LOGIC, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Configure `RotaryEncoder` to use the standard API
    pub fn into_standard_mode(self) -> RotaryEncoder<StandardMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: StandardMode::new(),
        }
    }
}

impl Default for StandardMode {
    fn default() -> Self {
        Self::new()
    }
}
