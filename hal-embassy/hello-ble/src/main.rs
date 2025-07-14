#![no_std]
#![no_main]
mod ble;
mod service;
use crate::ble::get_soft_device;
use crate::service::*;

use {defmt_rtt as _, panic_probe as _};

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use futures::future::{Either, select};
use futures::pin_mut;
use nrf_softdevice::Softdevice;
use nrf_softdevice::ble::{gatt_server, peripheral};

// Application must run at a lower priority than softdevice
fn nrf_config() -> embassy_nrf::config::Config {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = interrupt::Priority::P2;
    config.time_interrupt_priority = interrupt::Priority::P2;
    config
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // First we get the peripherals access crate.
    let _ = embassy_nrf::init(nrf_config());

    let sd = get_soft_device();
    let server = unwrap!(Server::new(sd));
    unwrap!(spawner.spawn(softdevice_task(sd)));

    loop {
        let config = peripheral::Config::default();

        let adv = ble::adv::get_adv();

        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);
        info!("advertising done! I have a connection.");

        let battery_fut = server.notify_battery_value(&conn);
        let gatt_fut = gatt_server::run(&conn, &server, |e| match e {
            ServerEvent::Bas(e) => match e {
                BatteryServiceEvent::BatteryLevelCccdWrite { notifications } => {
                    info!("battery notifications: {}", notifications)
                }
            },
        });

        pin_mut!(battery_fut);
        pin_mut!(gatt_fut);

        match select(battery_fut, gatt_fut).await {
            Either::Left((_, _)) => {
                info!("Battery Notification encountered an error and stopped!")
            }
            Either::Right((e, _)) => {
                info!("gatt_server run exited with error: {:?}", e);
            }
        };
    }
}

#[embassy_executor::task]
pub async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}
