//! # rotary-encoder
//! A rotary encoder library built for embedded applications

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use embedded_hal::digital::v2::InputPin;

/// Velocity type alias
pub type Velocity = f32;

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

/// State table for recognizing valid rotary encoder values
const STATES: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

/// Default angular velocity increasing factor
const DEFAULT_VELOCITY_INC_FACTOR: f32 = 0.2;

/// Default angular velocity decreasing factor
const DEFAULT_VELOCITY_DEC_FACTOR: f32 = 0.01;

/// Angular velocity action window duration in milliseconds
const DEFAULT_VELOCITY_ACTION_MS: i64 = 25;

/// Rotary Encoder
pub struct RotaryEncoder<DT, CLK> {
    pin_dt: DT,
    pin_clk: CLK,
    velocity: Velocity,
    velocity_inc_factor: f32,
    velocity_dec_factor: f32,
    velocity_action_ms: i64,
    pos_calc: i8,
    transition: u8,
    direction: Direction,
    previous_time: NaiveDateTime,
}

impl<DT, CLK> RotaryEncoder<DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Initiates a new Rotary Encoder, taking two InputPins [`InputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html).
    /// Optionally the behaviour of the angular velocity can be modified:
    ///   velocity_inc_factor: How quickly the velocity increases to 1.0.
    ///   velocity_dec_factor: How quickly the velocity decreases or cools-down
    ///   velocity_action_ms: The window of duration (milliseconds) that the velocity will increase
    pub fn new(
        pin_dt: DT,
        pin_clk: CLK,
        velocity_inc_factor: Option<f32>,
        velocity_dec_factor: Option<f32>,
        velocity_action_ms: Option<i64>,
    ) -> Self {
        return RotaryEncoder {
            pin_dt,
            pin_clk,
            pos_calc: 0,
            velocity: 0.0,
            velocity_inc_factor: velocity_inc_factor.unwrap_or(DEFAULT_VELOCITY_INC_FACTOR),
            velocity_dec_factor: velocity_dec_factor.unwrap_or(DEFAULT_VELOCITY_DEC_FACTOR),
            velocity_action_ms: velocity_action_ms.unwrap_or(DEFAULT_VELOCITY_ACTION_MS),
            transition: 0,
            direction: Direction::None,
            previous_time: NaiveDateTime::new(
                NaiveDate::from_ymd(2000, 1, 1),
                NaiveTime::from_hms_milli(0, 0, 0, 0),
            ),
        };
    }

    /// This function should be called periodically, either via a timer or the main loop.
    /// This function will reduce the angular velocity over time, the amount is configurable via the constructor
    pub fn tick(&mut self) {
        self.velocity -= self.velocity_dec_factor;
        if self.velocity < -1.0 {
            self.velocity = -1.0;
        }
    }

    /// Borrow a mutable reference to the underlying InputPins. This is useful for clearing hardware interrupts.
    pub fn borrow_pins(&mut self) -> (&mut DT, &mut CLK) {
        (&mut self.pin_dt, &mut self.pin_clk)
    }

    /// Release the underying resources such as the InputPins back to the initiator
    pub fn release(self) -> (DT, CLK) {
        (self.pin_dt, self.pin_clk)
    }

    /// Update the state machine of the RotaryEncoder. This should be called ideally from an interrupt vector
    /// when either the DT or CLK pins state changes. This function will update the RotaryEncoder's
    /// Direction and current Angular Velocity.
    pub fn update(&mut self, current_time: NaiveDateTime) {
        let dt_state = self.pin_dt.is_high().unwrap_or_default() as u8;
        let clk_state = self.pin_clk.is_high().unwrap_or_default() as u8;

        let current = (dt_state << 1) | clk_state;
        self.transition = (self.transition << 2) | current;
        let index = (self.transition & 0x0F) as usize;
        self.pos_calc = self.pos_calc + STATES[index];

        if self.pos_calc == 4 || self.pos_calc == -4 {
            if current_time.timestamp_millis() - self.previous_time.timestamp_millis()
                < self.velocity_action_ms
                && self.velocity < 1.0
            {
                self.velocity += self.velocity_inc_factor;
            }

            self.direction = if self.pos_calc == 4 {
                Direction::Clockwise
            } else {
                Direction::Anticlockwise
            };

            self.pos_calc = 0;
            return;
        }

        self.previous_time = current_time;
        self.direction = Direction::None;
    }

    /// Returns the current Direction of the RotaryEncoder
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Returns the current angular velocity of the RotaryEncoder
    /// The Angular Velocity is a value between 0.0 and 1.0
    /// This is useful for incrementing/decrementing a value in an exponential fashion
    pub fn velocity(&self) -> Velocity {
        return if self.velocity < 0.0 {
            0.0
        } else {
            self.velocity
        };
    }
}
