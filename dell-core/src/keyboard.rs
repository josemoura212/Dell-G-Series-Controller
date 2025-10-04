// Keyboard LED control implementation
// Translated from awelc.py

use crate::elc::{Action, Elc};
use crate::elc_constants::*;
use anyhow::{anyhow, Result};
use rusb::{DeviceHandle, GlobalContext};
use std::sync::{Arc, Mutex};

const SUPPORTED_VENDOR_ID: u16 = 0x187c;
const SUPPORTED_PRODUCT_IDS: [u16; 2] = [0x0550, 0x0551];

pub struct KeyboardController {
    elc: Elc,
    handle: Arc<Mutex<DeviceHandle<GlobalContext>>>,
}

impl KeyboardController {
    pub fn new(debug: bool) -> Result<Self> {
        let mut device_handle: Option<DeviceHandle<GlobalContext>> = None;

        for device in rusb::devices()?.iter() {
            let device_desc = device.device_descriptor()?;

            if device_desc.vendor_id() == SUPPORTED_VENDOR_ID
                && SUPPORTED_PRODUCT_IDS.contains(&device_desc.product_id())
            {
                match device.open() {
                    Ok(h) => {
                        device_handle = Some(h);
                        break;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }

        let device_handle = device_handle
            .ok_or_else(|| anyhow!("No supported device found (187c:0550 or 187c:0551)"))?;

        // Reset device
        let _ = device_handle.reset();

        // Detach kernel driver if active
        match device_handle.kernel_driver_active(0) {
            Ok(true) => {
                device_handle.detach_kernel_driver(0)?;
            }
            _ => {}
        }

        // Claim interface
        device_handle.claim_interface(0)?;

        let handle = Arc::new(Mutex::new(device_handle));
        let elc = Elc::new(handle.clone(), debug);

        Ok(Self { elc, handle })
    }

    fn battery_flashing(&self) -> Result<()> {
        self.elc.remove_animation(DC_LOW)?;
        self.elc.start_new_animation(DC_LOW)?;
        self.elc.start_series(&ZONES_ALL, 1)?;

        // Red flashing
        self.elc.add_action(&[Action::new(
            COLOR,
            DURATION_BATTERY_LOW,
            TEMPO_MIN,
            255,
            0,
            0,
        )])?;
        self.elc
            .add_action(&[Action::new(COLOR, DURATION_BATTERY_LOW, TEMPO_MIN, 0, 0, 0)])?;

        self.elc.finish_save_animation(DC_LOW)?;
        self.elc.set_default_animation(DC_LOW)?;

        Ok(())
    }

    pub fn set_static(&self, red: u8, green: u8, blue: u8) -> Result<()> {
        self.set_dim(0)?;

        // AC Sleep - Off
        self.apply_action(
            0,
            0,
            0,
            DURATION_MAX,
            TEMPO_MIN,
            AC_SLEEP,
            COLOR,
            &ZONES_ALL,
        )?;

        // AC Charged - Full brightness
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            TEMPO_MIN,
            AC_CHARGED,
            COLOR,
            &ZONES_ALL,
        )?;

        // AC Charging - Full brightness
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            TEMPO_MIN,
            AC_CHARGING,
            COLOR,
            &ZONES_ALL,
        )?;

        // DC Sleep - Off
        self.apply_action(
            0,
            0,
            0,
            DURATION_MAX,
            TEMPO_MIN,
            DC_SLEEP,
            COLOR,
            &ZONES_ALL,
        )?;

        // DC On - Half brightness
        self.apply_action(
            red / 2,
            green / 2,
            blue / 2,
            DURATION_MAX,
            TEMPO_MIN,
            DC_ON,
            COLOR,
            &ZONES_ALL,
        )?;

        self.battery_flashing()?;

        let _ = self.handle.lock().unwrap().reset();
        Ok(())
    }

    pub fn set_morph(&self, red: u8, green: u8, blue: u8, duration: u16) -> Result<()> {
        self.set_dim(0)?;

        // AC Sleep - Off
        self.apply_action(
            0,
            0,
            0,
            DURATION_MAX,
            TEMPO_MIN,
            AC_SLEEP,
            COLOR,
            &ZONES_ALL,
        )?;

        // AC Charged - Full brightness morph
        self.apply_morph_action(
            red, green, blue, duration, TEMPO_MIN, AC_CHARGED, &ZONES_ALL,
        )?;

        // AC Charging - Full brightness morph
        self.apply_morph_action(
            red,
            green,
            blue,
            duration,
            TEMPO_MIN,
            AC_CHARGING,
            &ZONES_ALL,
        )?;

        // DC Sleep - Off
        self.apply_action(
            0,
            0,
            0,
            DURATION_MAX,
            TEMPO_MIN,
            DC_SLEEP,
            COLOR,
            &ZONES_ALL,
        )?;

        // DC On - Half brightness morph
        self.apply_morph_action(
            red / 2,
            green / 2,
            blue / 2,
            duration,
            TEMPO_MIN,
            DC_ON,
            &ZONES_ALL,
        )?;

        self.battery_flashing()?;

        let _ = self.handle.lock().unwrap().reset();
        Ok(())
    }

    pub fn set_color_and_morph(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        red_morph: u8,
        green_morph: u8,
        blue_morph: u8,
        duration: u16,
    ) -> Result<()> {
        self.set_dim(0)?;

        // AC Sleep - Off
        self.apply_color_and_morph_action(0, 0, 0, 0, 0, 0, DURATION_MAX, TEMPO_MIN, AC_SLEEP)?;

        // AC Charged
        self.apply_color_and_morph_action(
            red,
            green,
            blue,
            red_morph,
            green_morph,
            blue_morph,
            duration,
            TEMPO_MIN,
            AC_CHARGED,
        )?;

        // AC Charging
        self.apply_color_and_morph_action(
            red,
            green,
            blue,
            red_morph,
            green_morph,
            blue_morph,
            duration,
            TEMPO_MIN,
            AC_CHARGING,
        )?;

        // DC Sleep - Off
        self.apply_color_and_morph_action(0, 0, 0, 0, 0, 0, DURATION_MAX, TEMPO_MIN, DC_SLEEP)?;

        // DC On - Half brightness
        self.apply_color_and_morph_action(
            red / 2,
            green / 2,
            blue / 2,
            red_morph / 2,
            green_morph / 2,
            blue_morph / 2,
            duration,
            TEMPO_MIN,
            DC_ON,
        )?;

        self.battery_flashing()?;

        let _ = self.handle.lock().unwrap().reset();
        Ok(())
    }

    pub fn remove_all_animations(&self) -> Result<()> {
        // Turn off LEDs by setting to black
        self.set_static(0, 0, 0)?;
        self.set_dim(100)?;

        let _ = self.handle.lock().unwrap().reset();
        Ok(())
    }

    pub fn set_dim(&self, level: u8) -> Result<()> {
        self.elc.dim(&ZONES_ALL, level)?;
        Ok(())
    }

    fn apply_action(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        duration: u16,
        tempo: u16,
        animation: u16,
        effect: u8,
        zones: &[u8],
    ) -> Result<()> {
        self.elc.remove_animation(animation)?;
        self.elc.start_new_animation(animation)?;
        self.elc.start_series(zones, 1)?;
        self.elc
            .add_action(&[Action::new(effect, duration, tempo, red, green, blue)])?;
        self.elc.finish_save_animation(animation)?;
        self.elc.set_default_animation(animation)?;
        Ok(())
    }

    fn apply_morph_action(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        duration: u16,
        tempo: u16,
        animation: u16,
        zones: &[u8],
    ) -> Result<()> {
        self.elc.remove_animation(animation)?;
        self.elc.start_new_animation(animation)?;
        self.elc.start_series(zones, 1)?;

        // Create morph transition: color -> intermediate -> opposite -> back
        self.elc.add_action(&[
            Action::new(MORPH, duration, tempo, red, green, blue),
            Action::new(MORPH, duration, tempo, 255 - red, 255 - green, 255 - blue),
            Action::new(MORPH, duration, tempo, red, green, blue),
        ])?;

        self.elc.finish_save_animation(animation)?;
        self.elc.set_default_animation(animation)?;
        Ok(())
    }

    fn apply_color_and_morph_action(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        red_morph: u8,
        green_morph: u8,
        blue_morph: u8,
        duration: u16,
        tempo: u16,
        animation: u16,
    ) -> Result<()> {
        // Numpad - Morph
        self.elc.remove_animation(animation)?;
        self.elc.start_new_animation(animation)?;
        self.elc.start_series(&ZONES_NP, 1)?;

        self.elc.add_action(&[
            Action::new(MORPH, duration, tempo, red_morph, green_morph, blue_morph),
            Action::new(MORPH, duration, tempo, green_morph, blue_morph, red_morph),
            Action::new(MORPH, duration, tempo, blue_morph, red_morph, green_morph),
        ])?;

        // Keyboard - Static
        self.elc.start_series(&ZONES_KB, 1)?;
        self.elc
            .add_action(&[Action::new(COLOR, duration, tempo, red, green, blue)])?;

        self.elc.finish_save_animation(animation)?;
        self.elc.set_default_animation(animation)?;

        Ok(())
    }
}
