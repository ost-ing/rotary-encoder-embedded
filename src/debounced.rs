use crate::table::{half::STATE_TABLE_HALF_STEPS, DIR_CCW, DIR_CW};
use crate::Direction;
use crate::RotaryEncoder;
use embedded_hal::digital::v2::InputPin;

/// Default decay factor
const DEFAULT_DECAY_FACTOR: f32 = 2.0;
/// Default debounce duration in milliseconds
const DEFAULT_DEBOUNCE_DURATION_MILLIS: u16 = 60;
/// Default decay increment value
const DEFAULT_DECAY_INCREMENT: f32 = 0.2;

/// Debounced Mode
/// Uses a table based state machine for the transitional states of the Rotary Encoder
/// but detects state changes on half steps and then uses a debouncing mechanism to filter
/// the half steps down to whole steps. As the user spins the rotary faster, the debouncing mechanism is reduced.
/// This functionality was written with Interrupts on Rising and Falling edges for both CLK and DT
pub struct DebouncedMode {
    /// The current state of the rotary encoder
    pub table_state: u8,
    /// The last moment (in milliseconds) when the rotary encoder yielded a `Direction` other than `Direction::None`
    pub last_update_millis: u64,
    /// How long the debounce is in effect for, this is reduced by the `decay` property
    pub debounce_duration_millis: u16,
    /// The instanteous decay value which reduces the debouncing effect
    pub decay: f32,
    /// How quickly the decay is reduced over time. A value of 2 means that it will decay in 1/2 a second
    pub decay_factor: f32,
    /// How quickly the decay increments
    pub decay_increment: f32,
}

impl<DT, CLK> RotaryEncoder<DebouncedMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Gets the current decay
    pub fn get_decay(&self) -> f32 {
        self.mode.decay
    }
    /// Gets the decay factor
    pub fn get_decay_factor(&self) -> f32 {
        self.mode.decay_factor
    }
    /// Sets the decay factor
    pub fn set_decay_factor(&mut self, decay_factor: f32) {
        self.mode.decay_factor = decay_factor;
    }
    /// Sets the decay increment value
    pub fn set_decay_increment(&mut self, decay_increment: f32) {
        self.mode.decay_increment = decay_increment;
    }
    /// Sets the debounce duration
    pub fn set_debounce_duration_millis(&mut self, debounce_duration_millis: u16) {
        self.mode.debounce_duration_millis = debounce_duration_millis;
    }

    /// Updates time based properties of the [`RotaryEncoder<DebounceMode, _, _>`]
    /// Must be called periodically, with `dt` being the sampling period between ticks
    /// For example, a timer with a refresh rate of 60Hz should call this function with a `dt` of 1/60.
    pub fn tick(&mut self, dt: f32) {
        let dec = self.mode.decay_factor * dt;
        self.mode.decay = (self.mode.decay - dec).max(0.0);
    }

    /// Update the state machine of the RotaryEncoder. This should be called ideally from an interrupt vector
    /// when either the DT or CLK pins state changes. This function will update the RotaryEncoder's Direction
    pub fn update(&mut self, millis: u64) {
        let dt_state = self.pin_dt.is_high().unwrap_or_default() as u8;
        let clk_state = self.pin_clk.is_high().unwrap_or_default() as u8;
        let pin_state = dt_state << 1 | clk_state;

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
                > (self.mode.debounce_duration_millis as u64
                    - (self.mode.debounce_duration_millis as f32 * self.mode.decay) as u64)
        {
            self.mode.decay = (self.mode.decay + self.mode.decay_increment).min(1.0);
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
    /// Configure `RotaryEncoder` to use the debounced API
    pub fn into_debounced_mode(self) -> RotaryEncoder<DebouncedMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: DebouncedMode {
                table_state: 0,
                last_update_millis: 0,
                decay: 0.0,
                decay_factor: DEFAULT_DECAY_FACTOR,
                debounce_duration_millis: DEFAULT_DEBOUNCE_DURATION_MILLIS,
                decay_increment: DEFAULT_DECAY_INCREMENT,
            },
            direction: Direction::None,
        }
    }
}
