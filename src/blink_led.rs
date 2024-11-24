#![no_std]
#![no_main]

use feather_m4 as bsp;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use bsp::hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    let mut blue_led = pins.d4.into_push_pull_output(); // led3
    let mut red_led = pins.d5.into_push_pull_output(); // led1
    let mut green_led = pins.d6.into_push_pull_output(); // led2

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        red_led.set_high().unwrap();
        green_led.set_low().unwrap();
        delay.delay_ms(250u16);
        blue_led.set_high().unwrap();
        delay.delay_ms(250u16);

        red_led.set_low().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_low().unwrap();
        delay.delay_ms(250u16);

        blue_led.set_high().unwrap();
        delay.delay_ms(250u16);
        blue_led.set_low().unwrap();
    }
}
