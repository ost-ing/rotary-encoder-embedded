# rotary-encoder-embedded

A rotary encoder library for embedded rust applications

- https://crates.io/crates/rotary-encoder-embedded

![rotary encoder](https://github.com/ostenning/images/blob/main/rotary-encoder.jpg?raw=true)

## features

- `no-std` support
- Suitable for gray-code incremental encoders
- Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.7/embedded_hal)


## modes

The `RotaryEncoder` can operate in a number of different modes, these modes provide different types of feature sets and are individually gated behind feature flags to keep the binary size to a minimum.
The following modes are currently provided:


| Feature flag  | Mode           | Desc.  |
| ------------- |-------------| -----|
| `full-step`        | `FullStepMode`              | Uses a full-step state machine for transitions |
| `debounced`        | `DebouncedMode`             | Uses a half-step state machine with dynamic debouncing |
| `angular-velocity` | `AngularVelocityMode`       | Uses a full-step state machine with additional angular-velocity calculations |

## `FullStepMode` example

```rust
fn main() -> ! {
    // Configure DT and CLK pins, typically pullup input
    let rotary_dt = gpio_pin_1.into_pull_up_input()
    let rotary_clk = gpio_pin_2.into_pull_up_input();
    // Initialize the rotary encoder
    let mut rotary_encoder = RotaryEncoder::new(
        rotary_dt,
        rotary_clk,
    ).into_fullstep_mode();

    // Application loop
    loop {
        // Update the encoder, which will compute its direction
        rotary_encoder.update();
        match rotary_encoder.direction() {
            Direction::Clockwise => {
                // Increment some value
            }
            Direction::AntiClockwise => {
                // Decrement some value
            }
            Direction::None => {
                // Do nothing
            }
        }
    }
}
```
## `DebounceMode` example

Trigger GPIO pin interrupts for both `DT` and `CLK` on both rising and falling edges

```rust
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder<_, _, FullStepMode>>>> = Mutex::new(RefCell::new(None));

fn main() -> ! {
    // Configure DT typically as pullup input & interrupt on rising/falling edges
    let mut rotary_dt = rotary_dt.into_pull_up_input();
    rotary_dt.make_interrupt_source(sys_cfg);
    rotary_dt.trigger_on_edge(exti, Edge::RisingFalling);
    rotary_dt.enable_interrupt(exti);
    // Configure CLK typically as pullup input & interrupt on rising/falling edges
    let mut rotary_clk = rotary_clk.into_pull_up_input();
    rotary_clk.make_interrupt_source(sys_cfg);
    rotary_clk.trigger_on_edge(exti, Edge::RisingFalling);
    rotary_clk.enable_interrupt(exti);

    // ... Configure a timer to interrupt at 60Hz

    // Initialize Rotary Encoder and safely store in static global
    interrupt::free(|cs| {
        ROTARY_ENCODER.borrow(cs).replace(Some(
            RotaryEncoder::new(
                rotary_dt,
                rotary_clk,
            ).into_debounced_mode()
        ));
    });

    
    loop {}
}

/// Called from Timer interrupt vector
fn handle_timer_interrupt() {
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
          const SAMPLE_PERIOD: f32 = 1.0 / 60.0;
          rotary_encoder.tick(SAMPLE_PERIOD);
        }
    });
}

/// Called from the GPIO interrupt vector
fn handle_rotary_interrupt() {
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            // Borrow the pins to clear the pending interrupt bit (which varies depending on HAL)
            let mut pins = rotary_encoder.pins_mut();
            pins.0.clear_interrupt_pending_bit();
            pins.1.clear_interrupt_pending_bit();
            
            // Update the encoder, which will compute its direction
            // current_time should be a monotonously rising time in ms (akin to Arduino's `millis()`)
            rotary_encoder.update(current_time);
            
            match rotary_encoder.direction() {
                Direction::Clockwise => {
                    // Increment some value
                }
                Direction::AntiClockwise => {
                    // Decrement some value
                }
                Direction::None => {
                    // Do nothing
                }
            }
        }
    });
}
```

## `AngularVelocityMode` example

```rust
fn main() -> ! {
    // Configure DT and CLK pins, typically pullup input
    let rotary_dt = gpio_pin_1.into_pull_up_input()
    let rotary_clk = gpio_pin_2.into_pull_up_input();
    // Initialize Rotary Encoder with Velocity functionality
    let mut rotary_encoder = RotaryEncoder::new(
        rotary_dt,
        rotary_clk,
    ).into_angular_velocity_mode();
    // Optional settings
    rotary_encoder.set_sensitivity(Sensitivity::Low);
    rotary_encoder.set_velocity_action_ms(5);       // The window of time that the velocity may increase
    rotary_encoder.set_velocity_inc_factor(0.2);    // How quickly the velocity increases over time
    rotary_encoder.set_velocity_dec_factor(0.01);   // How quickly the velocity decreases over time
    // Application loop
    loop {
        // Update the encoder which will compute its direction and velocity
        // As velocity is a function of time, we need the current time
        // current_time should be a monotonously rising time in ms (akin to Arduino's `millis()`)
        rotary_encoder.update(current_time);
        // Get the velocity
        let velocity = rotary_encoder.velocity();
        // Match the direction
        match rotary_encoder.direction() {
            Direction::Clockwise => {
                // Increment some value by some factor multiplied by velocity
            }
            Direction::AntiClockwise => {
                // Decrement some value by some factor multiplied by velocity
            }
            Direction::None => {
                // Do nothing
            }
        }
        // As velocity is a function of time, we need to reduce its value over time
        // This method could also be called from a Timer
        rotary_encoder.decay_velocity();
    }
}
```
