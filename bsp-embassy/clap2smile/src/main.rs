#![no_std]
#![no_main]

// use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    saadc::{self},
};
use embassy_time::{Duration, Timer};
use microbit_bsp::{
    Microbit,
    display::{Bitmap, Brightness, Frame},
    mic::Microphone,
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

const CLAP_THRESHOLD: u8 = 180;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();
    let mut led_matrix = board.display;

    let mut mic = Microphone::new(board.saadc, Irqs, board.microphone, board.micen);

    let smile_frame = Frame::<5, 5>::new([
        Bitmap::new(0b00000, 5),
        Bitmap::new(0b01010, 5),
        Bitmap::new(0b00000, 5),
        Bitmap::new(0b10001, 5),
        Bitmap::new(0b01110, 5),
    ]);

    led_matrix.set_brightness(Brightness::MAX);

    loop {
        // info!("Sound Level: {}", mic.sound_level().await);
        if mic.sound_level().await > CLAP_THRESHOLD {
            led_matrix
                .display(smile_frame, Duration::from_secs(1))
                .await;
        }
        Timer::after_millis(100).await;
    }
}
