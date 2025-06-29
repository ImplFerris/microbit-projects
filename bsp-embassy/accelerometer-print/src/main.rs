#![no_std]
#![no_main]

//use defmt::info;
use embassy_nrf::{self as hal, twim::Twim};
use hal::twim;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit_bsp::Microbit;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    defmt::info!("{:?}", panic_info);
    loop {}
}

hal::bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();

    let twim_config = twim::Config::default();
    let twim0 = Twim::new(
        board.twispi0,
        Irqs,
        board.i2c_int_sda,
        board.i2c_int_scl,
        twim_config,
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
        Timer::after(Duration::from_secs(5)).await;
    }
}
