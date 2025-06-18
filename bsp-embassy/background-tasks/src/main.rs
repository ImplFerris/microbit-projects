#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{gpio::Input, peripherals::PWM0, pwm::SimplePwm};
use embassy_time::{Duration, Timer};
use microbit_bsp::{
    Microbit,
    speaker::{NamedPitch, Pitch, PwmSpeaker},
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let board = Microbit::default();
    let mut display = board.display;

    let speaker = PwmSpeaker::new(SimplePwm::new_1ch(board.pwm0, board.speaker));

    spawner
        .spawn(button_task(board.btn_a, board.btn_b, speaker))
        .unwrap();

    loop {
        display
            .scroll_with_speed("EMBASSY", Duration::from_secs(10))
            .await;
        Timer::after_millis(300).await;
    }
}

#[embassy_executor::task]
async fn button_task(
    mut button_a: Input<'static>,
    mut button_b: Input<'static>,
    mut speaker: PwmSpeaker<'static, PWM0>,
) {
    loop {
        button_a.wait_for_low().await;
        speaker.start_note(Pitch::Named(NamedPitch::A4));

        button_b.wait_for_low().await;
        speaker.stop();
    }
}
