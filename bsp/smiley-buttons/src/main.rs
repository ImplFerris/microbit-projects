#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{display::blocking::Display, hal::Timer, Board};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let happy_face = [
        [0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];

    let sad_face = [
        [0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
    ];

    loop {
        let a_pressed = button_a.is_low().unwrap_or(false);
        let b_pressed = button_b.is_low().unwrap_or(false);

        if a_pressed {
            display.show(&mut timer, happy_face, 1000);
            timer.delay_ms(100);
        } else if b_pressed {
            display.show(&mut timer, sad_face, 1000);
            timer.delay_ms(100);
        }
    }
}
