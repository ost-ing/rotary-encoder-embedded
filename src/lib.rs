//! # rotary-encoder
//! A rotary encoder library built for embedded applications

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

mod table;

#[cfg(feature = "full-step")]
/// FullStepMode
pub mod full_step;

#[cfg(feature = "debounced")]
/// DebouncedMode
pub mod debounced;

#[cfg(feature = "angular-velocity")]
/// AngularVelocityMode
pub mod angular_velocity;

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
    pub fn pins_mut(&mut self) -> (&mut DT, &mut CLK) {
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
/// This is the plain `RotaryEncoder` with no business logic attached. In order to use the `RotaryEncoder` it must be initialized to a valid `Mode`
pub struct InitalizeMode {}
impl<DT, CLK> RotaryEncoder<InitalizeMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Initiates a new `RotaryEncoder` in `InitalizeMode`, taking two InputPins [`InputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html).
    pub fn new(pin_dt: DT, pin_clk: CLK) -> Self {
        RotaryEncoder {
            pin_dt,
            pin_clk,
            direction: Direction::None,
            mode: InitalizeMode {},
        }
    }
}
