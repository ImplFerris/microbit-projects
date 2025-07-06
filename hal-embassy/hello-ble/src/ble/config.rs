use {defmt_rtt as _, panic_probe as _};

use core::mem;

use nrf_softdevice::raw;

pub const DEVICE_NAME: &str = "implRust";

const fn clock_config() -> Option<raw::nrf_clock_lf_cfg_t> {
    Some(raw::nrf_clock_lf_cfg_t {
        source: raw::NRF_CLOCK_LF_SRC_RC as u8,
        // rc_ctiv: 16,
        rc_ctiv: 4,
        rc_temp_ctiv: 2,
        // accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
    })
}

const fn gap_conn_config() -> Option<raw::ble_gap_conn_cfg_t> {
    Some(raw::ble_gap_conn_cfg_t {
        conn_count: 2,
        event_length: 24,
    })
}

const fn gatt_conn_config() -> Option<raw::ble_gatt_conn_cfg_t> {
    Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 })
}

fn gap_device_name() -> Option<raw::ble_gap_cfg_device_name_t> {
    Some(raw::ble_gap_cfg_device_name_t {
        p_value: DEVICE_NAME.as_ptr() as _,
        current_len: DEVICE_NAME.len() as u16,
        max_len: DEVICE_NAME.len() as u16,
        write_perm: unsafe { mem::zeroed() },
        _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
            raw::BLE_GATTS_VLOC_STACK as u8,
        ),
    })
}

const fn gap_role_count() -> Option<raw::ble_gap_cfg_role_count_t> {
    Some(raw::ble_gap_cfg_role_count_t {
        adv_set_count: raw::BLE_GAP_ADV_SET_COUNT_DEFAULT as u8,
        periph_role_count: raw::BLE_GAP_ROLE_COUNT_PERIPH_DEFAULT as u8,
    })
}

const fn gatts_attr_tab_size() -> Option<raw::ble_gatts_cfg_attr_tab_size_t> {
    Some(raw::ble_gatts_cfg_attr_tab_size_t {
        attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
    })
}

// Softdevice config
pub fn softdevice_config() -> nrf_softdevice::Config {
    nrf_softdevice::Config {
        clock: clock_config(),
        conn_gap: gap_conn_config(),
        conn_gatt: gatt_conn_config(),
        gatts_attr_tab_size: gatts_attr_tab_size(),
        gap_role_count: gap_role_count(),
        gap_device_name: gap_device_name(),
        ..Default::default()
    }
}
