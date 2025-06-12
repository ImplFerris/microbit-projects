#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use microbit_bsp::Microbit;
use microbit_bsp::display::Brightness;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();
    let mut display = board.display;

    display.set_brightness(Brightness::new(5));

    loop {
        display
            .scroll_with_speed("EMBASSY", Duration::from_secs(10))
            .await;
        Timer::after_secs(1).await;
    }
}
