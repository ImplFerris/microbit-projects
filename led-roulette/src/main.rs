#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{self as _, display::blocking::Display};

use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Default)]
struct LedState {
    row: usize,
    col: usize,
    direction: Direction,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut led_matrix = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut led_state = LedState::default();

    loop {
        display.show(&mut timer, led_matrix, 50);
        display.clear();

        led_matrix[led_state.row][led_state.col] = 0;
        match led_state.direction {
            Direction::Right => {
                if led_state.col == 4 {
                    led_state.direction = Direction::Down;
                    led_state.row += 1;
                } else {
                    led_state.col += 1;
                }
            }
            Direction::Down => {
                if led_state.row == 4 {
                    led_state.direction = Direction::Left;
                    led_state.col = led_state.col.saturating_sub(1);
                } else {
                    led_state.row += 1;
                }
            }
            Direction::Left => {
                if led_state.col == 0 {
                    led_state.direction = Direction::Up;
                    led_state.row = led_state.row.saturating_sub(1);
                } else {
                    led_state.col = led_state.col.saturating_sub(1);
                }
            }
            Direction::Up => {
                if led_state.row == 0 {
                    led_state.direction = Direction::Right;
                    led_state.col += 1;
                } else {
                    led_state.row = led_state.row.saturating_sub(1);
                }
            }
        }

        led_matrix[led_state.row][led_state.col] = 1;
    }
}
