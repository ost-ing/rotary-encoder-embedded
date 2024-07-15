# rotary-encoder
A rotary encoder library for embedded rust applications

## features
 - Full no-std support
 - Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.3/embedded_hal)
 - Support for measuring angular velocity for non-linear control

## example

Note: Quick example based on the `stm32h7xx-hal`.

```
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder>>> = Mutex::new(RefCell::new(None));

fn main() -> ! {
    // ... Initialize DT and CLK pins as desired. Typically PullUp Push-Pull.
    // ... Initialize interrupt on rising and falling edge
    
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

    loop {
        // Will update time based properties such as the angular velocity.
        // Could also be executed from a timer. Modify the `velocity_dec_factor` to match its execution frequency
        rotary_encoder.tick();
    }
}

#[interrupt]
fn EXTI1() {
    // DT Rising or Falling edge interrupt
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            let current_time = ... // Get this NaiveDateTime based from your RTC or SysTick handler
            rotary_encoder.update(current_time);
            controls
                .rotary_encoder
                .borrow_pins()
                .0
                .clear_interrupt_pending_bit();
        }
    });
}

#[interrupt]
fn EXTI2() {
    // CLK Rising or Falling edge interrupt
    interrupt::free(|cs| {
        if let Some(ref mut rotary_encoder) = ROTARY_ENCODER.borrow(cs).borrow_mut().deref_mut() {
            let current_time = ... // Get this NaiveDateTime based from your RTC or SysTick handler
            rotary_encoder.update(current_time);
            controls
                .rotary_encoder
                .borrow_pins()
                .1
                .clear_interrupt_pending_bit();
        }
    });
}

```