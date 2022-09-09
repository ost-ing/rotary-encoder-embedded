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
| ------------- |----------------| -------|
| `standard`         | `StandardMode`              | Uses a state machine for transitions |
| `angular-velocity` | `AngularVelocityMode`       | Same as `standard` but with additional angular-velocity calculations |

## `StandardMode` example

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
    // ...timer initialize at 900Hz to poll the rotary encoder
    loop {}
}

fn timer_interrupt_handler() {
    // ... get rotary encoder 
    let rotary_encoder = ...
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
```

# A note about GPIO or Timer interrupt usage

I've experimented a lot with different combinations in order to make Rotary Encoders behave predictably because generally speaking they are fickle at best. From my experimentation I've learnt that using GPIO pin based interrupts generally isn't a good idea because they are more prone to noise and increase the risk of misfires and jumps.
Timers on the other hand provide a low pass filtering quality because they don't pick up higher frequency switching that GPIO interrupts do. I have found that using a Timer between 850-1000Hz seems to work best.
