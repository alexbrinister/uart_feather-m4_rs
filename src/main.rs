#![no_std]
#![no_main]

use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal::delay::DelayNs;
use hal::pac::{CorePeripherals, Peripherals};
use hal::time::Hertz;
use hal::timer::*;
use hal::timer_traits::InterruptDrivenTimer;
use hal::prelude::*;

use smart_leds::{
    colors::{RED, BLUE},
    brightness,
    SmartLedsWrite,
};

use ws2812_timer_delay::Ws2812;

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

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.mclk);
    InterruptDrivenTimer::start(&mut timer, Hertz::MHz(3).into_duration());

    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixel = Ws2812::new(timer, neopixel_pin);

    let mut blue_led = pins.d4.into_push_pull_output(); // led3
    let mut red_led = pins.d5.into_push_pull_output(); // led1
    let mut green_led = pins.d6.into_push_pull_output(); // led2

    loop {
        // Red
        let color = [RED; 1];
        neopixel.write(brightness(color.iter().cloned(), 32)).unwrap();

        red_led.set_high().unwrap();
        green_led.set_low().unwrap();
        DelayNs::delay_ms(&mut delay, 250u32);
        blue_led.set_high().unwrap();
        DelayNs::delay_ms(&mut delay, 250u32);

        let color = [BLUE; 1];
        neopixel.write(brightness(color.iter().cloned(), 32)).unwrap();

        red_led.set_low().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_low().unwrap();
        DelayNs::delay_ms(&mut delay, 250u32);

        blue_led.set_high().unwrap();
        DelayNs::delay_ms(&mut delay, 250u32);
        blue_led.set_low().unwrap();
    }
}
