#![no_main]
#![no_std]

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use nrf9160_hal as hal;
use nrf9160_hal::gpio::Level;
use rtt_target::{rprintln, rtt_init_print};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0_NS);
    let button = port0.p0_06.into_pullup_input().degrade();
    let mut led1 = port0.p0_02.into_push_pull_output(Level::Low);
    let mut led2 = port0.p0_03.into_push_pull_output(Level::Low);
    let mut led3 = port0.p0_04.into_push_pull_output(Level::Low);
    let mut led4 = port0.p0_05.into_push_pull_output(Level::Low);
    rprintln!("Blinky button demo starting");
    led1.set_high().unwrap();
    led2.set_high().unwrap();
    led3.set_high().unwrap();
    led4.set_high().unwrap();

    loop {
        if button.is_high().unwrap() {
            led1.set_low().unwrap();
            led2.set_low().unwrap();
            led3.set_low().unwrap();
            led4.set_low().unwrap();
            
        } else {
            led1.set_high().unwrap();
            led2.set_high().unwrap();
            led3.set_high().unwrap();
            led4.set_high().unwrap();
        }
    }
}
