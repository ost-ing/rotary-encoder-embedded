//! # rotary-encoder
//! A rotary encoder library built for embedded applications

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

/// Angular velocity mode
#[cfg(feature = "angular-velocity")]
pub mod angular_velocity;
/// Standard mode
#[cfg(feature = "standard")]
pub mod standard;

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

pub struct RotaryEncoder<LOGIC, DT, CLK> {
    pin_dt: DT,
    pin_clk: CLK,
    logic: LOGIC,
}

/// Core logic interface definition.
pub trait RotaryEncoderLogic {
    /// Updates the `RotaryEncoder`, updating the `direction` property
    fn update(&mut self, pin_dt: bool, pin_clk: bool);
    /// Gets the last detected direction
    fn direction(&self) -> Direction;
}

/// Common
impl<LOGIC, DT, CLK> RotaryEncoder<LOGIC, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
    LOGIC: RotaryEncoderLogic,
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
        self.logic.direction()
    }
    /// a
    pub fn update(&mut self) {
        self.logic.update(
            self.pin_dt.is_high().unwrap_or_default(),
            self.pin_clk.is_high().unwrap_or_default(),
        );
    }
}

/// InitializeMode
/// This is the plain `RotaryEncoder` with no business logic attached. In order to use the `RotaryEncoder` it must be initialized to a valid `Mode`
pub struct InitalizeMode {}

/// Empty core logic implementation for InitalizeMode
impl RotaryEncoderLogic for InitalizeMode {
    fn update(&mut self, _pin_dt: bool, _pin_clk: bool) {}

    fn direction(&self) -> Direction {
        Direction::None
    }
}

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
            logic: InitalizeMode {},
        }
    }
}

/// Generic struct for the core logic
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RotaryEncoderCore<MODE> {
    direction: Direction,
    mode: MODE,
}
