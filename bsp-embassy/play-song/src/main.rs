#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::pwm::SimplePwm;
use embassy_time::Timer;
use microbit_bsp::{
    Microbit,
    speaker::{Note, Pitch, PwmSpeaker},
};
use tinytones::{Tone, songs};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();
    let mut speaker = PwmSpeaker::new(SimplePwm::new_1ch(board.pwm0, board.speaker));

    let song = Tone::new(songs::happy_birthday::TEMPO, songs::happy_birthday::MELODY);
    loop {
        for (pitch, note_duration) in song.iter() {
            if pitch == tinytones::note::Pitch::Rest {
                Timer::after_millis(note_duration).await;
                continue;
            }

            speaker
                .play(&Note(
                    Pitch::Frequency(pitch.freq_u32()),
                    note_duration as u32,
                ))
                .await;
        }
        Timer::after_secs(5).await;
    }
}
