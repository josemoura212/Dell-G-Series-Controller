// HID Report implementation for USB control transfers
// Translated from hidreport.py

use anyhow::Result;
use rusb::{DeviceHandle, GlobalContext};

const REQUEST_TYPE_CLASS_OUT: u8 = 0x21;
const REQUEST_TYPE_CLASS_IN: u8 = 0xA1;
const SET_REPORT: u8 = 9;
const GET_REPORT: u8 = 1;
const TIMEOUT_MS: std::time::Duration = std::time::Duration::from_secs(5);

pub fn hid_set_output_report(
    handle: &DeviceHandle<GlobalContext>,
    report: &[u8],
    report_id: u8,
) -> Result<()> {
    handle.write_control(
        REQUEST_TYPE_CLASS_OUT,
        SET_REPORT,
        0x200 + report_id as u16,
        0x00,
        report,
        TIMEOUT_MS,
    )?;
    Ok(())
}

pub fn hid_get_input_report(
    handle: &DeviceHandle<GlobalContext>,
    length: usize,
    report_id: u8,
) -> Result<Vec<u8>> {
    let mut buffer = vec![0u8; length];
    handle.read_control(
        REQUEST_TYPE_CLASS_IN,
        GET_REPORT,
        0x100 + report_id as u16,
        0x00,
        &mut buffer,
        TIMEOUT_MS,
    )?;
    Ok(buffer)
}
