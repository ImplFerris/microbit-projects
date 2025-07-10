#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    saadc::{self, ChannelConfig, Saadc},
};
use microbit_bsp::{
    Microbit,
    display::{self, Brightness},
};

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

const THRESHOLD: i16 = 3500;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();
    let mut display = board.display;

    let config = saadc::Config::default();
    let channel_config = ChannelConfig::single_ended(board.p0);
    let mut adc = Saadc::new(board.saadc, Irqs, config, [channel_config]);

    #[rustfmt::skip]
    const FLASH: [u8; 5] = [
        0b00010,
        0b00100,
        0b01110,
        0b00100,
        0b01000,
    ];

    display.set_brightness(Brightness::MAX);
    loop {
        let mut buf = [0; 1];
        adc.sample(&mut buf).await;
        if buf[0] > THRESHOLD {
            display.apply(display::fonts::frame_5x5(&FLASH));
            display.render();
        } else {
            display.clear();
        }
    }
}
