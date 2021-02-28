# rotary-encoder-embedded
A rotary encoder library for embedded rust applications

- https://crates.io/crates/rotary-encoder-embedded

## features
 - Full no-std support
 - Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.3/embedded_hal)
 - Support for measuring angular velocity for non-linear control

## installation

Add the package via Cargo: `rotary-encoder-embedded = "0.0.1"`

## example

Note: Quick example based on the `stm32h7xx-hal`.

```
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder>>> = Mutex::new(RefCell::new(None));

fn main() -> ! {
    // ... Initialize DT and CLK pins as desired. Typically PullUp Push-Pull.
    // ... Initialize interrupt on rising and falling edge
    // ... Initialize a timer to periodically update rotary-encoder and other control systems

    interrupt::free(|cs| {
        ROTARY_ENCODER.borrow(cs).replace(Some(
            RotaryEncoder::new(
                rotary_dt,
                rotary_clk,
                Option::None, // optional velocity_inc_factor
                Option::None, // optional velocity_dec_factor
                Option::None, // optional velocity_action_ms
            )
        ));
    });

    loop {}
}

#[interrupt]
fn TIM1() {
    // Periodic timer update interrupt vector
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            // Note: This could also be run inside the main loop. 
            // The rotary_encoders internal velocity is decremented by `velocity_dec_factor` when
            // this function is called
            rotary_encoder.tick();
        }
    });
}

fn handle_rotary(rotary_encoder: &mut RotaryEncoder) {
    let current_time = ... // Get this NaiveDateTime based from your RTC or SysTick handler
    
    // Update the state of the rotary-encoder, computing its current direction and angular velocity
    rotary_encoder.update(current_time);

    // Get the rotary values
    let direction = rotary_encoder.direction();
    let velocity = rotary_encoder.velocity();

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