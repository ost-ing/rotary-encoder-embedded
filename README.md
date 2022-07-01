# rotary-encoder-embedded

A rotary encoder library for embedded rust applications

- https://crates.io/crates/rotary-encoder-embedded

![rotary encoder](https://github.com/ostenning/images/blob/main/rotary-encoder.jpg?raw=true)

## features

- Heapless & no standard library
- 2-bit gray code lookup table implementation
- Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.7/embedded_hal)
- Experimental support for measuring angular velocity for non-linear control (gated behind the `angular-velocity` feature flag)

## examples

Examples use the `stm32h7xx-hal` crate and are compatible with any project using `embedded-hal`. 
It is recommended to use the `interrupt driven example`.

### simple example

```rust
fn main() -> ! {
    // Configure DT and CLK pins, typically pullup input
    let rotary_dt = gpio_pin_1.into_pull_up_input()
    let rotary_clk = gpio_pin_2.into_pull_up_input();
    // Initialize the rotary encoder
    let mut rotary_encoder = RotaryEncoder::new(
        rotary_dt,
        rotary_clk,
    );
    // Optional, configure sensitivity if needed
    rotary_encoder.set_sensitivity(Sensitivity::Low);
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

### interrupt driven example

Trigger GPIO pin interrupts for both `DT` and `CLK` on both rising and falling edges

```rust
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder>>> = Mutex::new(RefCell::new(None));

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
    // Initialize Rotary Encoder and safely store in static global
    interrupt::free(|cs| {
        ROTARY_ENCODER.borrow(cs).replace(Some(
            RotaryEncoder::new(
                rotary_dt,
                rotary_clk,
            )
        ));
    });

    loop {}
}
/// Called from the GPIO interrupt vector
fn handle_rotary_interrupt() {
    // Retrieve Rotary Encoder from safely stored static global
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            // Borrow the pins to clear the pending interrupt bit (which varies depending on HAL)
            let mut pins = rotary_encoder.borrow_pins();
            pins.0.clear_interrupt_pending_bit();
            pins.1.clear_interrupt_pending_bit();
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
    });
}
```

### angular velocity example

For experimental angular velocity support use the `RotaryEncoderWithVelocity` struct. This functionality is gated behind the `angular-velocity` crate feature flag.

```rust
fn main() -> ! {
    // Configure DT and CLK pins, typically pullup input
    let rotary_dt = gpio_pin_1.into_pull_up_input()
    let rotary_clk = gpio_pin_2.into_pull_up_input();
    // Initialize Rotary Encoder with Velocity functionality
    let mut rotary_encoder = RotaryEncoderWithVelocity::new(
        rotary_dt,
        rotary_clk,
    );
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
