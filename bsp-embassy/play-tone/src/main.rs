#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::pwm::SimplePwm;
use microbit_bsp::{
    Microbit,
    speaker::{NamedPitch, Pitch, PwmSpeaker},
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();

    let mut button_a = board.btn_a;
    let mut button_b = board.btn_b;

    let mut speaker = PwmSpeaker::new(SimplePwm::new_1ch(board.pwm0, board.speaker));

    loop {
        button_a.wait_for_low().await;
        speaker.start_note(Pitch::Named(NamedPitch::A4));
        button_b.wait_for_low().await;
        speaker.stop();
    }

}
