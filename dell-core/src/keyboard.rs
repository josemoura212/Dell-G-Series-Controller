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

    pub fn set_pulse(&self, red: u8, green: u8, blue: u8, speed: u16) -> Result<()> {
        self.set_dim(0)?;
        let tempo = if speed == 0 { TEMPO_MIN } else { speed };

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

        // AC Charged - Pulse
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            tempo,
            AC_CHARGED,
            PULSE,
            &ZONES_ALL,
        )?;

        // AC Charging - Pulse
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            tempo,
            AC_CHARGING,
            PULSE,
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

        // DC On - Pulse
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            tempo,
            DC_ON,
            PULSE,
            &ZONES_ALL,
        )?;

        self.battery_flashing()?;
        let _ = self.handle.lock().unwrap().reset();
        Ok(())
    }

    pub fn set_zone_static(&self, zone: u8, red: u8, green: u8, blue: u8) -> Result<()> {
        // Implement simple single-zone static color
        // Note: setting one zone might reset others if we don't handle it carefully,
        // but for now let's allow it as a direct command.
        // Actually, to set one zone without affecting others, we'd need to read the current state
        // or build a comprehensive animation for all zones.
        // But the user asked for 4-zone control, and calls set_zone_colors (all 4).
        // set_zone_static is kept for completeness or if called individually.

        self.set_dim(0)?;

        // We'll just define the animation for THIS zone.
        // Important: this might clear other zones if the animation ID is shared and we overwrite it.
        // But apply_action overwrites the whole animation ID.
        // So passing just one zone to apply_action will make THAT zone inherit the action,
        // but what happens to others? They might default to nothing (off) or stay as is?
        // In Elc terms, if we start_series for zone 0, we define actions for zone 0.
        // If we don't mention zone 1, it might have 0 actions.

        // Use the colors/zones helper strategy for robust 4-zone control instead.
        // But for this function, let's just do what we can.

        // AC Charged
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            TEMPO_MIN,
            AC_CHARGED,
            COLOR,
            &[zone],
        )?;
        self.apply_action(
            red,
            green,
            blue,
            DURATION_MAX,
            TEMPO_MIN,
            AC_CHARGING,
            COLOR,
            &[zone],
        )?;
        self.apply_action(
            red / 2,
            green / 2,
            blue / 2,
            DURATION_MAX,
            TEMPO_MIN,
            DC_ON,
            COLOR,
            &[zone],
        )?;

        self.battery_flashing()?;
        let _ = self.handle.lock().unwrap().reset();
        Ok(())
    }

    pub fn set_four_zone_colors(&self, colors: &[[u8; 3]; 4]) -> Result<()> {
        self.set_dim(0)?;

        // Helper to apply multi-zone colors to a specific animation ID
        let apply_multizone = |animation: u16, dim_factor: u8| -> Result<()> {
            self.elc.remove_animation(animation)?;
            self.elc.start_new_animation(animation)?;

            for (zone_idx, color) in colors.iter().enumerate() {
                let zone = zone_idx as u8;
                let r = color[0] / dim_factor;
                let g = color[1] / dim_factor;
                let b = color[2] / dim_factor;

                self.elc.start_series(&[zone], 1)?;
                self.elc
                    .add_action(&[Action::new(COLOR, DURATION_MAX, TEMPO_MIN, r, g, b)])?;
            }

            self.elc.finish_save_animation(animation)?;
            self.elc.set_default_animation(animation)?;
            Ok(())
        };

        // Apply to all power states
        apply_multizone(AC_SLEEP, 255)?; // Off (div by 255 -> 0)
                                         // Actually just use 0,0,0 explicitly for sleep to be safe
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

        apply_multizone(AC_CHARGED, 1)?;
        apply_multizone(AC_CHARGING, 1)?;
        apply_multizone(DC_ON, 2)?; // Half brightness

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
