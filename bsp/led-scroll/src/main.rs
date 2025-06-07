#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
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

    // 5x5 representation of 'R'
    let r_char = [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 0, 0, 1, 0],
    ];

    let mut offset = 0;

    loop {
        let mut frame = [[0; 5]; 5];

        for row in 0..5 {
            for col in 0..5 {
                let char_col = col as isize + offset - 4;

                if char_col >= 0 && char_col < 5 {
                    frame[row][col] = r_char[row][char_col as usize];
                } else {
                    frame[row][col] = 0;
                }
            }
        }

        display.show(&mut timer, frame, 500);
        timer.delay_ms(100);

        offset += 1;
        if offset > 8 {
            offset = 0;
        }
    }
}
