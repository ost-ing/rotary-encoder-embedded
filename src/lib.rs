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
pub struct DefaultMode;
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
            mode: DefaultMode {},
        }
    }
}
