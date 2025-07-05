use defmt::{info, unwrap};
use embassy_time::Timer;
use nrf_softdevice::ble::Connection;

#[nrf_softdevice::gatt_service(uuid = "180f")]
pub struct BatteryService {
    #[characteristic(uuid = "2a19", read, notify)]
    battery_level: i16,
}

#[nrf_softdevice::gatt_server]
pub struct Server {
    bas: BatteryService,
}

impl Server {
    pub async fn notify_battery_value(&self, connection: &Connection) {
        let mut battery_level = 100;
        let mut charging = false;
        loop {
            if battery_level < 20 {
                charging = true;
            } else if battery_level >= 100 {
                charging = false;
            }

            if charging {
                battery_level += 5;
            } else {
                battery_level -= 5;
            }

            match self.bas.battery_level_notify(connection, &battery_level) {
                Ok(_) => info!("Battery Level: {=i16}", &battery_level),
                Err(_) => unwrap!(self.bas.battery_level_set(&battery_level)),
            };

            Timer::after_secs(2).await
        }
    }
}
