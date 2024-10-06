use core::pin::Pin;

use embassy_nrf::gpio::Output;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use rtt_target::rprintln;

use crate::buttons::ButtonDirection;

const NUM_COLS: usize = 5;

pub struct LedRow {
    col: [Output<'static>; NUM_COLS],
    active_col: usize,
}

impl LedRow {
    pub fn new(col: [Output<'static>; NUM_COLS]) -> Self {
        Self { col, active_col: 0 }
    }

    pub fn toggle(&mut self) {
        rprintln!("Blinking LED {}", self.active_col);
        #[cfg(feature = "trigger-overflow")]
        {
            use crate::time::Ticker;
            let time = Ticker::now();
            rprintln!(
                "Time: 0x{:x} ticks, {}",
                time.ticks(),
                time.duration_since_epoch().to_millis(),
            );
        }
        self.col[self.active_col].toggle();
    }

    pub fn shift(&mut self, direction: ButtonDirection) {
        rprintln!("Button press detected...");
        // turn off the previous led
        self.col[self.active_col].set_high();

        // get new led location to turn on based on the direction
        self.active_col = match direction {
            ButtonDirection::Left => match self.active_col {
                0 => 4,
                _ => self.active_col - 1,
            },
            ButtonDirection::Right => (self.active_col + 1) % NUM_COLS,
        };

        // switch of the led so that it will switch one when the state changed to Toggle
        self.col[self.active_col].set_high();
    }
}
