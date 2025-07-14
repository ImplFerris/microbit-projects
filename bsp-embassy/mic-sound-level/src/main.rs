#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    saadc::{self},
};
use embassy_time::Timer;
use microbit_bsp::{Microbit, mic::Microphone};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();

    let mut mic = Microphone::new(board.saadc, Irqs, board.microphone, board.micen);
    loop {
        info!("Sound Level: {}", mic.sound_level().await);
        Timer::after_millis(100).await;
    }
}
