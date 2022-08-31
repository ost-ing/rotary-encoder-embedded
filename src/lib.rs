//! # rotary-encoder
//! A rotary encoder library built for embedded applications

// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]

mod table;

#[cfg(feature = "angular-velocity")]
pub mod angular;
#[cfg(feature = "debounce")]
pub mod debounced;

#[cfg(feature = "debounce")]
use debounced::DebouncedMode;
use embedded_hal::digital::v2::InputPin;
use table::{DIR_CCW, DIR_CW, STATE_TABLE_FULL_STEPS};

/// Direction of Rotary Encoder rotation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// No Direction is specified,
    None,
    /// Clockwise direction
    Clockwise,
    /// Anti-clockwise direction
    Anticlockwise,
}

/// Rotary Encoder
pub struct RotaryEncoder<MODE, DT, CLK> {
    pin_dt: DT,
    pin_clk: CLK,
    direction: Direction,
    mode: MODE,
}

/// Common
impl<DT, CLK, MODE> RotaryEncoder<MODE, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Borrow a mutable reference to the underlying InputPins. This is useful for clearing hardware interrupts.
    pub fn borrow_pins_mut(&mut self) -> (&mut DT, &mut CLK) {
        (&mut self.pin_dt, &mut self.pin_clk)
    }

    /// Release the underying resources such as the InputPins back to the initiator
    pub fn release(self) -> (DT, CLK) {
        (self.pin_dt, self.pin_clk)
    }

    /// Returns the current Direction of the RotaryEncoder
    pub fn direction(&self) -> Direction {
        self.direction
    }
}

/// Default Mode
pub struct DefaultMode {
    table_state: u8,
}

impl<DT, CLK> RotaryEncoder<DefaultMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Initiates a new Rotary Encoder, taking two InputPins [`InputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html).
    pub fn new(pin_dt: DT, pin_clk: CLK) -> Self {
        RotaryEncoder {
            pin_dt,
            pin_clk,
            direction: Direction::None,
            mode: DefaultMode { table_state: 0 },
        }
    }
    /// Updates the Rotary Encoder, updating the `direction` property
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
