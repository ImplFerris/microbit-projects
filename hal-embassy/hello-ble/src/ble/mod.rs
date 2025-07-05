pub mod adv;
pub mod config;

use crate::ble::config::softdevice_config;

use {defmt_rtt as _, panic_probe as _};

use nrf_softdevice::Softdevice;

pub fn get_soft_device() -> &'static mut Softdevice {
    let sd_config = softdevice_config();
    Softdevice::enable(&sd_config)
}
