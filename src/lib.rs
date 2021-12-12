#![no_std]

// Extract commonly used functionality
extern crate feather_m4 as hal;

// ws2812 = neopixel
extern crate ws2812_timer_delay as ws2812;

// Set up the Hardware Abstraction Library
use hal::{
    prelude::*,
    timer::TimerCounter,
    clock::GenericClockController,
    gpio::{Output, PushPull, Pb3},
    TC3,
    MCLK,
};

// Creates a new Neopixel object to control the Neopixel on the Feather M4
// Take from snd's rust-feather-m4 experiment
pub fn new_neopixel<'a>(
    timer_counter_3: TC3,
    main_clock: &mut MCLK,
    clock_controller: &mut GenericClockController,
    neopixel_pin: &'a mut Pb3<Output<PushPull>>
) -> ws2812::Ws2812<'a, TimerCounter<hal::TC3>, Pb3<Output<PushPull>>> {
    // Get a reference to clock generator 0
    let clock_generator_0 = clock_controller.gclk0();

    // Configure clock generator for use with either timer counter 2 or 3 peripherals.
    // TODO error handling
    let clock_configured_for_timer_counter_3 =
        clock_controller.tc2_tc3(&clock_generator_0).unwrap();

    // hardware timer counter 3
    let mut timer_counter = TimerCounter::tc3_(
        &clock_configured_for_timer_counter_3,
        timer_counter_3,
        main_clock,
    );

    // repeatedly counts down from 3 MHz
    timer_counter.start(3_000_000u32.hz());

    // create the neopixel object
    ws2812::Ws2812::new(timer_counter, neopixel_pin)
}
