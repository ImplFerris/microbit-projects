#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;

use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use rtt_target::{rprintln, rtt_init_print};

use crate::text::Text;
mod text;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let input = match Text::try_from("RUST") {
        Ok(input) => input,
        Err(err) => {
            rprintln!("Error: {:?}", err);
            panic!("Error");
        }
    };

    loop {
        for m in input.chars.clone() {
            display.show(&mut timer, m, 1000);
        }

        display.clear();
        timer.delay_ms(250_u32);
    }
}
