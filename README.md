# rotary-encoder-embedded

A rotary encoder library for embedded rust applications

- https://crates.io/crates/rotary-encoder-embedded

![rotary encoder](https://github.com/ostenning/images/blob/main/rotary-encoder.jpg?raw=true)

## features

- Full no-std support
- Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.3/embedded_hal)
- Support for measuring angular velocity for non-linear control

## examples

All examples are based on the `stm32h7xx-hal`, but are compatible with any project using `embedded-hal`. 

Its highly recommended to use the GPIO Interrupt driven implementation. Interrupts should occur on rising and falling edges for both `CLK` and `DT`.

### simple implementation
A simple example using the main loop. Its highly recommended to use the GPIO Interrupt driven implementation.

```rust
fn main() -> ! {
    // ... Initialize DT and CLK pins as desired. Typically PullUp Push-Pull.
    let mut rotary_encoder = RotaryEncoder::new(
        rotary_dt,
        rotary_clk,
    );

    // Optional: to configure sensitivity if needed
    rotary_encoder.set_sensitivity(Sensitivity::Low);

    loop {
        // Update the encoder, which will compute its direction
        rotary_encoder.update();

        // Get the rotary values
        let direction = rotary_encoder.direction();
        if direction == Direction::Clockwise {
            // Increment some value
        } else if direction == Direction::AntiClockwise {
            // Decrement some value
        }
    }
}
```

### interrupt driven example
Trigger GPIO pin interrupts for both DT and CLK on both rising and falling edges

```rust
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder>>> = Mutex::new(RefCell::new(None));

fn main() -> ! {
    // ... Initialize DT and CLK pins as desired. Typically PullUp Push-Pull.
    // ... Initialize interrupt on rising and falling edge
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

fn handle_rotary(rotary_encoder: &mut RotaryEncoder) {
    // Update the state of the rotary-encoder
    rotary_encoder.update();

    // Get the rotary values
    let direction = rotary_encoder.direction();
    if direction == Direction::Clockwise {
        // Increment some value
    } else if direction == Direction::AntiClockwise {
        // Decrement some value
    }
}

#[interrupt]
fn EXTI1() {
    // DT rising or falling edge interrupt
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            // Clear DT GPIO EXTI interrupt
            rotary_encoder
                .borrow_pins()
                .0
                .clear_interrupt_pending_bit();

            handle_rotary(rotary_encoder);
        }
    });
}

#[interrupt]
fn EXTI2() {
    // CLK rising or falling edge interrupt
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            // Clear CLK GPIO EXTI interrupt
            rotary_encoder
                .borrow_pins()
                .1
                .clear_interrupt_pending_bit();

            handle_rotary(rotary_encoder);
        }
    });
}
```

### angular velocity example
If angular velocity is required, which increases complexity a little, see below: 

```rust
fn main() -> ! {
    // ... Initialize DT and CLK pins as desired. Typically PullUp Push-Pull.
    let mut rotary_encoder = RotaryEncoderWithVelocity::new(
        rotary_dt,
        rotary_clk,
    );

    // Optional settings
    rotary_encoder.set_sensitivity(Sensitivity::Low);
    rotary_encoder.set_velocity_action_ms(5);       // The window of time that the velocity may increase
    rotary_encoder.set_velocity_inc_factor(0.2);    // How quickly the velocity increases over time
    rotary_encoder.set_velocity_dec_factor(0.01);   // How quickly the velocity decreases over time

    loop {
        // Update the encoder which will compute its direction and velocity.
        // As velocity is a function of time, we need the current time.
        // current_time should be derived from the RTC and SysTick.
        rotary_encoder.update(current_time);

        // Get the direction & velocity
        let direction = rotary_encoder.direction();
        let velocity = rotary_encoder.velocity();

        if direction == Direction::Clockwise {
            // Increment some value
        } else if direction == Direction::AntiClockwise {
            // Decrement some value
        }

        // As velocity is a function of time, we need to reduce its value over time.
        // This value could also be called from a Timer.
        rotary_encoder.decay_velocity();
    }
}
```
