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

    let led_matrix = [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 0, 0, 1, 0],
    ];

    let mut col_count = 0;  

    loop {
        let mut frame = [[0; 5]; 5]; 

        let start_col = 4 - col_count;

        for col_index in 0..=col_count {
            // From end to beginning 
            let frame_col = start_col + col_index;
            for row in 0..5 {
                frame[row][frame_col] = led_matrix[row][col_index];

            }
        }

        display.show(&mut timer, frame, 500);
        timer.delay_ms(60);

        col_count += 1;

        if col_count >= 5 {
            col_count = 0;
        }
    }

}