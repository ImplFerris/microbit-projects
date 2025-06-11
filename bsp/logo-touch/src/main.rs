#![no_std]
#![no_main]

use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer};

use cortex_m_rt::entry;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);
    let led_matrix = [
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
    ];

    // Pin connected to the Logo
    let mut touch_input = board.pins.p1_04.into_floating_input();

    loop {
        if touch_input.is_low().unwrap() {
            display.show(&mut timer, led_matrix, 500);
        } else {
            display.clear();
        }
    }
}
