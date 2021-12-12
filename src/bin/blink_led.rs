//! alternates off-board LED on DIO 5 for 4 s period, 50% duty cycle
#![no_std]
#![no_main]

// set panicking behavior to halt
extern crate panic_halt;
extern crate feather_m4 as hal;

use hal::{
    prelude::*,
    entry,
    Peripherals,
    CorePeripherals,
    clock::GenericClockController,
    delay::Delay,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    // Create the DIO pin (pin 5)
    let mut pins = feather_m4::Pins::new(peripherals.PORT);
    let mut led_pin = pins.d5.into_open_drain_output(&mut pins.port);

    let mut clock_controller = GenericClockController::with_external_32kosc(
        // generic clock controller
        peripherals.GCLK,
        // main clock
        &mut peripherals.MCLK,
        // 32 Hz oscillators control
        &mut peripherals.OSC32KCTRL,
        // oscillators control
        &mut peripherals.OSCCTRL,
        // non-volatile memory controller
        &mut peripherals.NVMCTRL,
    );

    // SYST is the system timer
    let mut delay = Delay::new(core_peripherals.SYST, &mut clock_controller);

    loop {
        led_pin.set_high();
        delay.delay_ms(2_000u16);
        led_pin.set_low();
        delay.delay_ms(2_000u16);
    }
}
