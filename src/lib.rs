//! # rotary-encoder
//! A rotary encoder library built for embedded applications

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use embedded_hal::digital::v2::InputPin;

/// Angular velocity api
pub mod angular_velocity;
/// Standard api
pub mod standard;

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
    mode: MODE,
    pin_dt: DT,
    pin_clk: CLK,
}

/// Common
impl<MODE, DT, CLK> RotaryEncoder<MODE, DT, CLK>
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

    /// Borrow the underlying mode
    pub fn mode(&mut self) -> &mut MODE {
        &mut self.mode
    }
}

/// InitializeMode
/// This is the plain `RotaryEncoder` with no business logic attached. In order to use the `RotaryEncoder` it must be initialized to a valid `Mode`
pub struct InitalizeMode;

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
            mode: InitalizeMode {},
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{angular_velocity::AngularVelocityMode, standard::StandardMode, RotaryEncoder};
    use embedded_hal_mock::eh0::pin::Mock;

    fn get_pins() -> (Mock, Mock) {
         (Mock::new([]), Mock::new([]))
    }

    #[test]
    fn standard_mode_api() {
        let (dt, clk) = get_pins();

        // Standard mode can be used with embedded-hal pins
        let mut encoder = RotaryEncoder::new(dt, clk).into_standard_mode();
        let _dir = encoder.update();

        // Or it can be used directly, bypassing the pins 
        let mut raw_encoder = StandardMode::new();
        let _dir = raw_encoder.update(true, false);
    }

    #[test]
    fn angular_velocity_mode_api() {
        let (dt, clk) = get_pins();

        // Angular velocity mode can be used with embedded-hal pins
        let mut encoder = RotaryEncoder::new(dt, clk).into_angular_velocity_mode();
        let _dir = encoder.update(0);

        // Or it can be used directly, bypassing the pins 
        let mut raw_encoder = AngularVelocityMode::new();
        let _dir = raw_encoder.update(true, false, 100);
    }
}
