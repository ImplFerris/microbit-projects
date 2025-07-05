use crate::ble::config::DEVICE_NAME;

use {defmt_rtt as _, panic_probe as _};

use nrf_softdevice::ble::{
    advertisement_builder::{
        Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList, ServiceUuid16,
    },
    peripheral,
};

static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
    .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
    .services_16(ServiceList::Complete, &[ServiceUuid16::BATTERY])
    .full_name(DEVICE_NAME)
    .build();

static SCAN_DATA: [u8; 0] = [];

pub fn get_adv() -> peripheral::ConnectableAdvertisement<'static> {
    peripheral::ConnectableAdvertisement::ScannableUndirected {
        adv_data: &ADV_DATA,
        scan_data: &SCAN_DATA,
    }
}
