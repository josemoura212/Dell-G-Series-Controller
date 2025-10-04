// ELC (LED Controller) implementation
// Translated from elc.py

use crate::elc_constants::*;
use crate::hid_report::{hid_get_input_report, hid_set_output_report};
use anyhow::{anyhow, Result};
use rusb::{DeviceHandle, GlobalContext};

#[derive(Debug, Clone)]
pub struct Action {
    pub effect: u8,
    pub duration: u16,
    pub tempo: u16,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Action {
    pub fn new(effect: u8, duration: u16, tempo: u16, red: u8, green: u8, blue: u8) -> Self {
        Self {
            effect,
            duration,
            tempo,
            red,
            green,
            blue,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.effect);
        bytes.extend_from_slice(&self.duration.to_be_bytes());
        bytes.extend_from_slice(&self.tempo.to_be_bytes());
        bytes.push(self.red);
        bytes.push(self.green);
        bytes.push(self.blue);
        bytes
    }
}

pub struct Elc {
    handle: std::sync::Arc<std::sync::Mutex<DeviceHandle<GlobalContext>>>,
}

impl Elc {
    pub fn new(
        handle: std::sync::Arc<std::sync::Mutex<DeviceHandle<GlobalContext>>>,
        _debug: bool,
    ) -> Self {
        Self { handle }
    }

    fn build_command(&self, fragment: &[u8]) -> Vec<u8> {
        let mut bytes = vec![0x03];
        bytes.extend_from_slice(fragment);
        bytes.resize(33, 0x00);
        bytes
    }

    fn run_command(&self, fragment: &[u8]) -> Result<Vec<u8>> {
        let bytes = self.build_command(fragment);
        let handle = self.handle.lock().unwrap();
        hid_set_output_report(&handle, &bytes, 0)?;
        let reply = hid_get_input_report(&handle, 33, 0)?;

        Ok(reply)
    }

    #[allow(unused)]
    pub fn get_version(&self) -> Result<(u8, u8, u8)> {
        let fragment = vec![ELC_QUERY, GET_VERSION];
        let reply = self.run_command(&fragment)?;
        Ok((reply[3], reply[4], reply[5]))
    }

    #[allow(dead_code)]
    pub fn get_animation_count(&self) -> Result<(u8, u16)> {
        let fragment = vec![ELC_QUERY, GET_ANIMATION_COUNT];
        let reply = self.run_command(&fragment)?;
        Ok((reply[3], u16::from_le_bytes([reply[4], reply[5]])))
    }

    pub fn start_new_animation(&self, animation: u16) -> Result<()> {
        let command = if animation < 0x5b || animation > 0x60 {
            USER_ANIMATION
        } else {
            POWER_ANIMATION
        };

        let mut fragment = vec![command];
        fragment.extend_from_slice(&START_NEW.to_be_bytes());
        fragment.extend_from_slice(&animation.to_be_bytes());

        self.run_command(&fragment)?;
        Ok(())
    }

    pub fn finish_save_animation(&self, animation: u16) -> Result<()> {
        let command = if animation < 0x5b || animation > 0x60 {
            USER_ANIMATION
        } else {
            POWER_ANIMATION
        };

        let mut fragment = vec![command];
        fragment.extend_from_slice(&FINISH_SAVE.to_be_bytes());
        fragment.extend_from_slice(&animation.to_be_bytes());

        self.run_command(&fragment)?;
        Ok(())
    }

    pub fn remove_animation(&self, animation: u16) -> Result<()> {
        let command = if animation < 0x5b || animation > 0x60 {
            USER_ANIMATION
        } else {
            POWER_ANIMATION
        };

        let mut fragment = vec![command];
        fragment.extend_from_slice(&REMOVE.to_be_bytes());
        fragment.extend_from_slice(&animation.to_be_bytes());

        self.run_command(&fragment)?;
        Ok(())
    }

    pub fn set_default_animation(&self, animation: u16) -> Result<()> {
        let command = if animation < 0x5b || animation > 0x60 {
            USER_ANIMATION
        } else {
            POWER_ANIMATION
        };

        let mut fragment = vec![command];
        fragment.extend_from_slice(&SET_DEFAULT.to_be_bytes());
        fragment.extend_from_slice(&animation.to_be_bytes());

        self.run_command(&fragment)?;
        Ok(())
    }

    pub fn start_series(&self, zones: &[u8], loop_count: u8) -> Result<()> {
        let mut fragment = vec![START_SERIES, loop_count];
        fragment.extend_from_slice(&(zones.len() as u16).to_be_bytes());
        fragment.extend_from_slice(zones);

        self.run_command(&fragment)?;
        Ok(())
    }

    pub fn add_action(&self, actions: &[Action]) -> Result<()> {
        if actions.len() > 3 {
            return Err(anyhow!("Too many actions in a single start action"));
        }

        let mut fragment = vec![ADD_ACTION];
        for action in actions {
            fragment.extend_from_slice(&action.to_bytes());
        }

        self.run_command(&fragment)?;
        Ok(())
    }

    pub fn dim(&self, zones: &[u8], dimming: u8) -> Result<()> {
        let mut fragment = vec![DIMMING, dimming];
        fragment.extend_from_slice(&(zones.len() as u16).to_be_bytes());
        fragment.extend_from_slice(zones);

        self.run_command(&fragment)?;
        Ok(())
    }

    #[allow(unused)]
    pub fn set_color(&self, zones: &[u8], red: u8, green: u8, blue: u8) -> Result<()> {
        let mut fragment = vec![SET_COLOR, red, green, blue];
        fragment.extend_from_slice(&(zones.len() as u16).to_be_bytes());
        fragment.extend_from_slice(zones);

        self.run_command(&fragment)?;
        Ok(())
    }
}
