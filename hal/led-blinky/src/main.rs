#![no_std]
#![no_main]

use cortex_m_rt::entry;

// Embedded HAL traits
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

// nRF HAL
use nrf52833_hal::gpio::Level;
use nrf52833_hal::pac::Peripherals;
use nrf52833_hal::{self as hal, timer::Timer};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut timer0 = Timer::new(peripherals.TIMER0);

    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::High);

    loop {
        timer0.delay_ms(500);
        row1.set_high().unwrap();

        timer0.delay_ms(500);
        row1.set_low().unwrap();
    }
}
