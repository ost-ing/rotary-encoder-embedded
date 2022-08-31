/// Default angular velocity increasing factor
const DEFAULT_VELOCITY_INC_FACTOR: f32 = 0.2;
/// Default angular velocity decreasing factor
const DEFAULT_VELOCITY_DEC_FACTOR: f32 = 0.01;
/// Angular velocity action window duration in milliseconds
const DEFAULT_VELOCITY_ACTION_MS: u64 = 25;
/// Velocity type, the value is between 0.0 and 1.0
pub type Velocity = f32;

/// Rotary Encoder with velocity
pub struct RotaryEncoderWithVelocity<DT, CLK> {
    inner: RotaryEncoder<DT, CLK>,
    velocity: Velocity,
    velocity_inc_factor: f32,
    velocity_dec_factor: f32,
    velocity_action_ms: u64,
    previous_time: u64,
}

impl<DT, CLK> RotaryEncoderWithVelocity<DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Initiates a new Rotary Encoder with velocity, taking two InputPins [`InputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html).
    /// Optionally the behaviour of the angular velocity can be modified:
    pub fn new(pin_dt: DT, pin_clk: CLK) -> Self {
        RotaryEncoderWithVelocity {
            inner: RotaryEncoder::new(pin_dt, pin_clk),
            velocity: 0.0,
            velocity_inc_factor: DEFAULT_VELOCITY_INC_FACTOR,
            velocity_dec_factor: DEFAULT_VELOCITY_DEC_FACTOR,
            velocity_action_ms: DEFAULT_VELOCITY_ACTION_MS,
            previous_time: 0,
        }
    }

    /// Set the velocity_inc_factor. How quickly the velocity increases to 1.0.
    pub fn set_velocity_inc_factor(&mut self, inc_factor: f32) {
        self.velocity_inc_factor = inc_factor;
    }

    /// Set the velocity_dec_factor. How quickly the velocity decreases or cools-down
    pub fn set_velocity_dec_factor(&mut self, dec_factor: f32) {
        self.velocity_dec_factor = dec_factor;
    }

    /// Set the velocity_action_ms. The window of duration (milliseconds) that the velocity will increase
    pub fn set_velocity_action_ms(&mut self, action_ms: u64) {
        self.velocity_action_ms = action_ms;
    }

    /// This function should be called periodically, either via a timer or the main loop.
    /// This function will reduce the angular velocity over time, the amount is configurable via the constructor
    pub fn decay_velocity(&mut self) {
        self.velocity -= self.velocity_dec_factor;
        if self.velocity < 0.0 {
            self.velocity = 0.0;
        }
    }

    /// Borrow a mutable reference to the underlying InputPins. This is useful for clearing hardware interrupts.
    pub fn borrow_pins(&mut self) -> (&mut DT, &mut CLK) {
        self.inner.borrow_pins()
    }

    /// Set the sensitivity of the rotary encoder
    pub fn set_sensitivity(&mut self, sensitivity: Sensitivity) {
        self.inner.sensitivity = sensitivity;
    }

    /// Borrow a reference to the underlying RotaryEncoder. Useful for configuring the RotaryEncoder
    pub fn borrow_inner(&mut self) -> &mut RotaryEncoder<DT, CLK> {
        &mut self.inner
    }

    /// Release the underying resources such as the InputPins back to the initiator
    pub fn release(self) -> (DT, CLK) {
        self.inner.release()
    }

    /// Update the state machine of the RotaryEncoder. This should be called ideally from an interrupt vector
    /// when either the DT or CLK pins state changes. This function will update the RotaryEncoder's
    /// Direction and current Angular Velocity.
    /// * `current_time` - Current timestamp in ms (strictly monotonously increasing)
    pub fn update(&mut self, current_time: u64) {
        self.inner.update();

        if self.inner.direction() != Direction::None {
            if current_time - self.previous_time < self.velocity_action_ms && self.velocity < 1.0 {
                self.velocity += self.velocity_inc_factor;
                if self.velocity > 1.0 {
                    self.velocity = 1.0;
                }
            }
            return;
        }

        self.previous_time = current_time;
    }

    /// Returns the current Direction of the RotaryEncoder
    pub fn direction(&self) -> Direction {
        self.inner.direction
    }

    /// Returns the current angular velocity of the RotaryEncoder
    /// The Angular Velocity is a value between 0.0 and 1.0
    /// This is useful for incrementing/decrementing a value in an exponential fashion
    pub fn velocity(&self) -> Velocity {
        self.velocity
    }
}
