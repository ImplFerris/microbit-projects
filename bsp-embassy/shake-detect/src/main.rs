#![no_std]
#![no_main]

use defmt::info;
use embassy_nrf::{self as hal, pwm::SimplePwm, twim::Twim};
use hal::twim;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit_bsp::{
    Microbit,
    speaker::{NamedPitch, Pitch, PwmSpeaker},
};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    info!("{:?}", panic_info);
    loop {}
}

hal::bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

const SHAKE_THRESHOLD_MG: i32 = 2000;

const SHAKE_THRESHOLD_SQUARED: i64 = (SHAKE_THRESHOLD_MG * SHAKE_THRESHOLD_MG) as i64;

fn detect_shake(x: i32, y: i32, z: i32) -> bool {
    let mag_sq = x as i64 * x as i64 + y as i64 * y as i64 + z as i64 * z as i64;
    mag_sq > SHAKE_THRESHOLD_SQUARED
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // let p = embassy_nrf::init(Default::default());
    let board = Microbit::default();

    let twim_config = twim::Config::default();
    let twim0 = Twim::new(
        board.twispi0,
        Irqs,
        board.i2c_int_sda,
        board.i2c_int_scl,
        twim_config,
    );

    let mut speaker = PwmSpeaker::new(SimplePwm::new_1ch(board.pwm0, board.speaker));

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
            let x = data.x_mg();
            let y = data.y_mg();
            let z = data.z_mg();
            if detect_shake(x, y, z) {
                // info!("SHAKE => x:{}, y:{}, z:{}", x, y, z);
                speaker.start_note(Pitch::Named(NamedPitch::A4));
                Timer::after_millis(100).await;
                speaker.stop();
            }
        }
        Timer::after_millis(50).await;
    }
}
