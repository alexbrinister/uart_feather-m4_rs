#![no_std]
#![no_main]

// BSP stuff
use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

// Hardware Abstraction Libs
use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};

// Neopixel support
use smart_leds::{
    brightness,
    colors::{BLUE, RED},
    SmartLedsWrite,
};
use ws2812_timer_delay::Ws2812;

use hal::prelude::*;
use hal::time::Hertz;
use hal::timer::*;
use hal::timer_traits::InterruptDrivenTimer;

mod common;

use rucos_cortex_m as rucos;

fn led_task(_: u32) -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = &clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.mclk);
    InterruptDrivenTimer::start(&mut timer, Hertz::MHz(3).into_duration());

    let pins = bsp::Pins::new(peripherals.port);

    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixel = Ws2812::new(timer, neopixel_pin);

    let mut blue_led = pins.d4.into_push_pull_output(); // led3
    let mut red_led = pins.d5.into_push_pull_output(); // led1
    let mut green_led = pins.d6.into_push_pull_output(); // led2

    loop {
        // Red
        let color = [RED; 1];
        neopixel
            .write(brightness(color.iter().cloned(), 32))
            .unwrap();

        red_led.set_high().unwrap();
        green_led.set_low().unwrap();
        rucos::sleep(250u64);
        blue_led.set_high().unwrap();
        rucos::sleep(250u64);

        let color = [BLUE; 1];
        neopixel
            .write(brightness(color.iter().cloned(), 32))
            .unwrap();

        red_led.set_low().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_low().unwrap();
        rucos::sleep(250u64);

        blue_led.set_high().unwrap();
        rucos::sleep(250u64);
        blue_led.set_low().unwrap();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut resources = common::setup();

    let mut idle_stack: [u8; common::IDLE_STACK_SIZE] = [0; common::IDLE_STACK_SIZE];
    rucos::init(&mut idle_stack, None);

    let mut led_task_stack: [u8; common::TASK_STACK_SIZE] = [0; common::TASK_STACK_SIZE];
    rucos::create(0, 0, &mut led_task_stack, led_task, None);

    rucos::start(
        &mut resources.scb,
        &mut resources.systick,
        Hertz::MHz(120).to_Hz(), // TODO: another way to do this, from the actual clock source
                                 // itself?
    );
}
