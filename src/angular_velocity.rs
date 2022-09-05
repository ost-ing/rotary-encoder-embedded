use crate::Direction;
use crate::RotaryEncoder;
use embedded_hal::digital::v2::InputPin;

/// Default angular velocity increasing factor
const DEFAULT_VELOCITY_INC_FACTOR: f32 = 0.2;
/// Default angular velocity decreasing factor
const DEFAULT_VELOCITY_DEC_FACTOR: f32 = 0.01;
/// Angular velocity action window duration in milliseconds
const DEFAULT_VELOCITY_ACTION_MS: u64 = 25;
/// Velocity type, the value is between 0.0 and 1.0
pub type Velocity = f32;

// For debouncing of pins, use 0x0f (b00001111) and 0x0c (b00001100) etc.
const PIN_MASK: u8 = 0x03;
const PIN_EDGE: u8 = 0x02;

/// AngularVelocityMode
/// Uses the full-step table with additional angular-velocity measurement
pub struct AngularVelocityMode {
    /// The pin state
    pin_state: [u8; 2],
    /// The instantaneous velocity
    velocity: Velocity,
    /// The increasing factor
    velocity_inc_factor: f32,
    /// The decreasing factor
    velocity_dec_factor: f32,
    /// The action window
    velocity_action_ms: u64,
    /// The last timestamp in mS
    previous_time_millis: u64,
}

impl<DT, CLK> RotaryEncoder<AngularVelocityMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Set the velocity_inc_factor. How quickly the velocity increases to 1.0.
    pub fn set_velocity_inc_factor(&mut self, inc_factor: f32) {
        self.mode.velocity_inc_factor = inc_factor;
    }

    /// Set the velocity_dec_factor. How quickly the velocity decreases or cools-down
    pub fn set_velocity_dec_factor(&mut self, dec_factor: f32) {
        self.mode.velocity_dec_factor = dec_factor;
    }

    /// Set the velocity_action_ms. The window of duration (milliseconds) that the velocity will increase
    pub fn set_velocity_action_ms(&mut self, action_ms: u64) {
        self.mode.velocity_action_ms = action_ms;
    }

    /// This function should be called periodically, either via a timer or the main loop.
    /// This function will reduce the angular velocity over time, the amount is configurable via the constructor
    pub fn decay_velocity(&mut self) {
        self.mode.velocity -= self.mode.velocity_dec_factor;
        if self.mode.velocity < 0.0 {
            self.mode.velocity = 0.0;
        }
    }

    /// Update the state machine of the RotaryEncoder. This should be called ideally from an interrupt vector
    /// when either the DT or CLK pins state changes. This function will update the RotaryEncoder's
    /// Direction and current Angular Velocity.
    /// * `current_time` - Current timestamp in ms (strictly monotonously increasing)
    pub fn update(&mut self, current_time_millis: u64) {
        self.mode.pin_state[0] =
            (self.mode.pin_state[0] << 1) | self.pin_dt.is_high().unwrap_or_default() as u8;
        self.mode.pin_state[1] =
            (self.mode.pin_state[1] << 1) | self.pin_clk.is_high().unwrap_or_default() as u8;

        let a = self.mode.pin_state[0] & PIN_MASK;
        let b = self.mode.pin_state[1] & PIN_MASK;

        let mut dir: Direction = Direction::None;

        if a == PIN_EDGE && b == 0x00 {
            dir = Direction::Anticlockwise;
        } else if b == PIN_EDGE && a == 0x00 {
            dir = Direction::Clockwise;
        }
        self.direction = dir;

        if self.direction != Direction::None {
            if current_time_millis - self.mode.previous_time_millis < self.mode.velocity_action_ms
                && self.mode.velocity < 1.0
            {
                self.mode.velocity += self.mode.velocity_inc_factor;
                if self.mode.velocity > 1.0 {
                    self.mode.velocity = 1.0;
                }
            }
            return;
        }

        self.mode.previous_time_millis = current_time_millis;
    }

    /// Returns the current angular velocity of the RotaryEncoder
    /// The Angular Velocity is a value between 0.0 and 1.0
    /// This is useful for incrementing/decrementing a value in an exponential fashion
    pub fn velocity(&self) -> Velocity {
        self.mode.velocity
    }
}

impl<DT, CLK, MODE> RotaryEncoder<MODE, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Configure `RotaryEncoder` to use the AngularVelocityMode API
    pub fn into_angular_velocity_mode(self) -> RotaryEncoder<AngularVelocityMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: AngularVelocityMode {
                pin_state: [0xFF, 2],
                velocity: 0.0,
                previous_time_millis: 0,
                velocity_action_ms: DEFAULT_VELOCITY_ACTION_MS,
                velocity_dec_factor: DEFAULT_VELOCITY_DEC_FACTOR,
                velocity_inc_factor: DEFAULT_VELOCITY_INC_FACTOR,
            },
            direction: Direction::None,
        }
    }
}
