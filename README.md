# rotary-encoder-embedded

A rotary encoder library for embedded rust applications

![Tests](https://github.com/ostenning/rotary-encoder-embedded/actions/workflows/build.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rotary-encoder-embedded.svg)](https://crates.io/crates/rotary-encoder-embedded)
[![Documentation](https://docs.rs/rotary-encoder-embedded/badge.svg)](https://docs.rs/rotary-encoder-embedded)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.46+-blue.svg)

![rotary encoder](https://github.com/ostenning/images/blob/main/rotary-encoder.jpg?raw=true)

## features

- `no-std` support
- Suitable for gray-code incremental encoders
- Multiple modes encluding `StandardMode`, `QuadratureTableMode` and `VelocityMode`
- Implemented with embedded-hal (https://docs.rs/embedded-hal/1.0.0/embedded_hal/)


```rust
fn main() -> ! {
    // Configure DT and CLK pins, typically pullup input
    let rotary_dt = gpio_pin_1.into_pull_up_input()
    let rotary_clk = gpio_pin_2.into_pull_up_input();

    // Initialize the rotary encoder
    let mut rotary_encoder = RotaryEncoder::new(
        rotary_dt,
        rotary_clk,
    ).into_standard_mode();
    
    // Now you can update the state of the rotary encoder and get a direction value. Call this from an update routine, timer task or interrupt
    let _dir = rotary_encoder.update();

    // Alternatively if you want to access the encoder without embedded-hal pin traits and use boolean states, you can use the mode directly:
    let mut raw_encoder = StandardMode::new();
    let _dir = raw_encoder.update(true, false);

    // ...timer initialize at 900Hz to poll the rotary encoder
    loop {}
}

fn timer_interrupt_handler() {
    // ... get rotary encoder 
    let rotary_encoder = ...

    // Update the encoder, which will compute and return its direction
    match rotary_encoder.update() {
        Direction::Clockwise => {
            // Increment some value
        }
        Direction::Anticlockwise => {
            // Decrement some value
        }
        Direction::None => {
            // Do nothing
        }
    }
}
```
