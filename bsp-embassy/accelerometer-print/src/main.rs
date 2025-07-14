#![no_std]
#![no_main]

use embassy_nrf::{self as hal, twim::Twim};
use hal::twim;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit_bsp::Microbit;
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

hal::bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();

    let twim_config = twim::Config::default();
    static RAM_BUFFER: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);

    let twim0 = Twim::new(
        board.twispi0,
        Irqs,
        board.i2c_int_sda,
        board.i2c_int_scl,
        twim_config,
        RAM_BUFFER.take(),
    );

    let mut sensor = Lsm303agr::new_with_i2c(twim0);
    sensor.init().await.unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut Delay,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .await
        .unwrap();

    loop {
        if sensor.accel_status().await.unwrap().xyz_new_data() {
            let data = sensor.acceleration().await.unwrap();
            // milli-g values
            let x = data.x_mg();
            let y = data.y_mg();
            let z = data.z_mg();
            defmt::info!("x:{}, y:{}, z:{}", x, y, z);
        }
        Timer::after_secs(1).await;
    }
}
