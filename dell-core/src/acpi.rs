// ACPI control for power and fan management
// Translated from the ACPI parts of main.py

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::process::Command;
use log::{debug, info, error};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaptopModel {
    G15_5530,
    G15_5520,
    G15_5525,
    G15_5515,
    G15_5511,
    G16_7620,
    G16_7630,
    AlienwareM16R1,
    Unknown,
}

impl LaptopModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::G15_5530 => "G15 5530",
            Self::G15_5520 => "G15 5520",
            Self::G15_5525 => "G15 5525",
            Self::G15_5515 => "G15 5515",
            Self::G15_5511 => "G15 5511",
            Self::G16_7620 => "G16 7620",
            Self::G16_7630 => "G16 7630",
            Self::AlienwareM16R1 => "Alienware M16 R1",
            Self::Unknown => "Unknown",
        }
    }

    pub fn supports_keyboard(&self) -> bool {
        !matches!(self, Self::G16_7630 | Self::Unknown)
    }
}

const INTEL_ACPI_PATH: &str = "\\_SB.AMWW.WMAX";
const AMD_ACPI_PATH: &str = "\\_SB.AMW3.WMAX";

pub struct AcpiController {
    acpi_path: String,
    acpi_call_dict: HashMap<String, Vec<String>>,
    pub power_modes: HashMap<String, String>,
    pub model: LaptopModel,
}

impl AcpiController {
    pub fn new() -> Result<Self> {
        let mut controller = Self {
            acpi_path: INTEL_ACPI_PATH.to_string(),
            acpi_call_dict: HashMap::new(),
            power_modes: Self::default_power_modes(),
            model: LaptopModel::Unknown,
        };

        controller.setup_acpi_calls();
        controller.detect_model()?;

        Ok(controller)
    }

    fn default_power_modes() -> HashMap<String, String> {
        let mut modes = HashMap::new();
        modes.insert("USTT_Balanced".to_string(), "0xa0".to_string());
        modes.insert("USTT_Performance".to_string(), "0xa1".to_string());
        modes.insert("USTT_Quiet".to_string(), "0xa3".to_string());
        modes.insert("USTT_FullSpeed".to_string(), "0xa4".to_string());
        modes.insert("USTT_BatterySaver".to_string(), "0xa5".to_string());
        modes.insert("G Mode".to_string(), "0xab".to_string());
        modes.insert("Manual".to_string(), "0x0".to_string());
        modes
    }

    fn setup_acpi_calls(&mut self) {
        self.acpi_call_dict.insert(
            "get_laptop_model".to_string(),
            vec!["0x1a".to_string(), "0x02".to_string(), "0x02".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_power_mode".to_string(),
            vec!["0x14".to_string(), "0x0b".to_string(), "0x00".to_string()],
        );
        self.acpi_call_dict.insert(
            "set_power_mode".to_string(),
            vec!["0x15".to_string(), "0x01".to_string()],
        );
        self.acpi_call_dict.insert(
            "toggle_G_mode".to_string(),
            vec!["0x25".to_string(), "0x01".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_G_mode".to_string(),
            vec!["0x25".to_string(), "0x02".to_string()],
        );
        self.acpi_call_dict.insert(
            "set_fan1_boost".to_string(),
            vec!["0x15".to_string(), "0x02".to_string(), "0x32".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_fan1_boost".to_string(),
            vec!["0x14".to_string(), "0x0c".to_string(), "0x32".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_fan1_rpm".to_string(),
            vec!["0x14".to_string(), "0x05".to_string(), "0x32".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_cpu_temp".to_string(),
            vec!["0x14".to_string(), "0x04".to_string(), "0x01".to_string()],
        );
        self.acpi_call_dict.insert(
            "set_fan2_boost".to_string(),
            vec!["0x15".to_string(), "0x02".to_string(), "0x33".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_fan2_boost".to_string(),
            vec!["0x14".to_string(), "0x0c".to_string(), "0x33".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_fan2_rpm".to_string(),
            vec!["0x14".to_string(), "0x05".to_string(), "0x33".to_string()],
        );
        self.acpi_call_dict.insert(
            "get_gpu_temp".to_string(),
            vec!["0x14".to_string(), "0x04".to_string(), "0x06".to_string()],
        );
    }


    fn set_model(&mut self, model: LaptopModel, is_amd: bool) {
        self.model = model;
        self.acpi_path = if is_amd {
            AMD_ACPI_PATH.to_string()
        } else {
            INTEL_ACPI_PATH.to_string()
        };
        self.apply_model_patch();
        info!("Model detected: {} ({})", model.as_str(), if is_amd { "AMD" } else { "Intel" });
    }

    fn detect_model(&mut self) -> Result<()> {
        // Try DMI detection first (works without root)
        if let Ok(output) = Command::new("cat")
            .arg("/sys/class/dmi/id/product_name")
            .output()
        {
            let product = String::from_utf8_lossy(&output.stdout).to_lowercase();
            info!("DMI product name: {}", product.trim());

            let detected = match () {
                _ if product.contains("g15") && product.contains("5530") => Some((LaptopModel::G15_5530, false)),
                _ if product.contains("g15") && product.contains("5520") => Some((LaptopModel::G15_5520, false)),
                _ if product.contains("g15") && product.contains("5525") => Some((LaptopModel::G15_5525, true)),
                _ if product.contains("g15") && product.contains("5515") => Some((LaptopModel::G15_5515, true)),
                _ if product.contains("g15") && product.contains("5511") => Some((LaptopModel::G15_5511, false)),
                _ if product.contains("g16") && product.contains("7630") => Some((LaptopModel::G16_7630, false)),
                _ if product.contains("g16") && product.contains("7620") => Some((LaptopModel::G16_7620, false)),
                _ => None,
            };

            if let Some((model, is_amd)) = detected {
                self.set_model(model, is_amd);
                return Ok(());
            }

            if product.contains("g15") || product.contains("g16") {
                info!("Generic Dell G-series detected, probing ACPI interface...");
            }
        }

        // Fallback: ACPI probing - try Intel path first
        self.acpi_path = INTEL_ACPI_PATH.to_string();
        match self.acpi_call("get_laptop_model", None, None) {
            Ok(model_str) => {
                let model = model_str.trim();
                debug!("ACPI model probe (Intel): {}", model);
                match model {
                    "0x0" => { self.set_model(LaptopModel::G15_5530, false); return Ok(()); }
                    "0x12c0" => { self.set_model(LaptopModel::G15_5520, false); return Ok(()); }
                    "0xc80" => { self.set_model(LaptopModel::G15_5511, false); return Ok(()); }
                    _ => {}
                }
            }
            Err(e) => debug!("Intel ACPI probe failed: {}", e),
        }

        // Try AMD path
        self.acpi_path = AMD_ACPI_PATH.to_string();
        match self.acpi_call("get_laptop_model", None, None) {
            Ok(model_str) => {
                let model = model_str.trim();
                debug!("ACPI model probe (AMD): {}", model);
                match model {
                    "0x12c0" => { self.set_model(LaptopModel::G15_5525, true); return Ok(()); }
                    "0xc80" => { self.set_model(LaptopModel::G15_5515, true); return Ok(()); }
                    _ => info!("Could not determine specific model, using generic configuration"),
                }
            }
            Err(e) => {
                debug!("AMD ACPI probe failed: {}", e);
                info!("Could not detect model via ACPI, using generic configuration");
            }
        }

        Ok(())
    }

    fn apply_model_patch(&mut self) {
        match self.model {
            LaptopModel::G15_5530 | LaptopModel::G15_5520 => {
                self.power_modes.remove("USTT_FullSpeed");
            }
            LaptopModel::G15_5515 => {
                self.power_modes.remove("USTT_Balanced");
                self.power_modes.remove("USTT_Performance");
                self.power_modes.remove("USTT_Quiet");
                self.power_modes.remove("USTT_FullSpeed");
                self.power_modes.remove("USTT_BatterySaver");
            }
            LaptopModel::G15_5511 => {
                self.power_modes.remove("USTT_FullSpeed");
                self.power_modes.remove("USTT_BatterySaver");
                self.power_modes.insert("USTT_Cool".to_string(), "0xa2".to_string());
            }
            LaptopModel::G16_7630 => {
                self.power_modes.remove("USTT_FullSpeed");
            }
            _ => {}
        }
    }

    pub fn acpi_call(&mut self, cmd: &str, arg1: Option<&str>, arg2: Option<&str>) -> Result<String> {
        let args = self
            .acpi_call_dict
            .get(cmd)
            .ok_or_else(|| anyhow!("Unknown ACPI command: {}", cmd))?;

        let cmd_str = match args.len() {
            4 => format!("echo \"{} 0 {} {{{}, {}, {}, 0x00}}\" > /proc/acpi/call", self.acpi_path, args[0], args[1], args[2], args[3]),
            3 => {
                let a1 = arg1.unwrap_or("0x00");
                format!("echo \"{} 0 {} {{{}, {}, {}, 0x00}}\" > /proc/acpi/call", self.acpi_path, args[0], args[1], args[2], a1)
            }
            2 => {
                let a1 = arg1.unwrap_or("0x00");
                let a2 = arg2.unwrap_or("0x00");
                format!("echo \"{} 0 {} {{{}, {}, {}, 0x00}}\" > /proc/acpi/call", self.acpi_path, args[0], args[1], a1, a2)
            }
            _ => return Err(anyhow!("Invalid ACPI call format")),
        };

        debug!("ACPI command [{}]: {}", cmd, cmd_str);

        let output = Command::new("pkexec")
            .arg("sh")
            .arg("-c")
            .arg(&format!("{}; cat /proc/acpi/call", cmd_str))
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("dismissed") || stderr.contains("Not authorized") {
                error!("Autorizacao cancelada pelo usuario para: {}", cmd);
                return Err(anyhow!("Autorizacao cancelada. Aceite a janela de autorizacao para continuar."));
            }
            error!("ACPI call '{}' failed: {}", cmd, stderr);
            return Err(anyhow!("Falha no comando ACPI '{}': {}", cmd, stderr));
        }

        let result = String::from_utf8_lossy(&output.stdout);
        debug!("ACPI result [{}]: {}", cmd, result.trim());

        let lines: Vec<&str> = result.lines().collect();
        if let Some(last_line) = lines.last() {
            let trimmed = last_line.trim().trim_end_matches('%').trim_matches('\'');
            Ok(trimmed.to_string())
        } else {
            Err(anyhow!("Sem resposta do comando ACPI '{}'", cmd))
        }
    }

    pub fn set_power_mode(&mut self, mode: &str) -> Result<()> {
        info!("Setting power mode: {} (ACPI path: {})", mode, self.acpi_path);

        let mode_value = self
            .power_modes
            .get(mode)
            .ok_or_else(|| anyhow!("Modo desconhecido: '{}'. Modos disponiveis: {:?}", mode, self.power_modes.keys().collect::<Vec<_>>()))?
            .clone();

        info!("Power mode '{}' -> ACPI value: {}", mode, mode_value);
        self.acpi_call("set_power_mode", Some(&mode_value), None)?;
        info!("Power mode '{}' aplicado com sucesso", mode);
        Ok(())
    }

    #[allow(unused)]
    pub fn get_power_mode(&mut self) -> Result<String> {
        self.acpi_call("get_power_mode", None, None)
    }

    pub fn set_fan_boost(&mut self, fan_id: u8, boost: u8) -> Result<()> {
        let fan_code = if fan_id == 1 { "32" } else { "33" };
        let cmd = format!("echo \"{} 0 0x15 {{0x02, 0x{}, 0x{:02X}, 0x00}}\" > /proc/acpi/call",
                         self.acpi_path, fan_code, boost);

        debug!("Setting fan {} boost to {}: {}", fan_id, boost, cmd);

        let output = Command::new("pkexec")
            .arg("sh")
            .arg("-c")
            .arg(&format!("{}; cat /proc/acpi/call", cmd))
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("dismissed") || stderr.contains("Not authorized") {
                return Err(anyhow!("Autorizacao cancelada. Aceite a janela de autorizacao."));
            }
            error!("Fan boost failed (fan {}): {}", fan_id, stderr);
            Err(anyhow!("Falha no controle do ventilador {}: {}", fan_id, stderr))
        }
    }

    pub fn get_fan_rpm(&mut self, fan_id: u8) -> Result<u32> {
        let cmd = match fan_id {
            1 => "get_fan1_rpm",
            2 => "get_fan2_rpm",
            _ => return Err(anyhow!("Invalid fan ID")),
        };
        
        let result = self.acpi_call(cmd, None, None)?;
        
        // Check for error values
        if result == "0xffffffff" {
            return Ok(0); // Return 0 for unavailable sensors
        }
        
        // Clean the result: remove 0x prefix, null bytes, and any non-hex characters
        let clean_result = result
            .trim_start_matches("0x")
            .trim_end_matches('\0')
            .trim()
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect::<String>();
        
        // Parse hex directly
        let rpm = u32::from_str_radix(&clean_result, 16)?;
        Ok(rpm)
    }

    pub fn get_temp(&mut self, sensor: &str) -> Result<u32> {
        let cmd = match sensor {
            "cpu" => "get_cpu_temp",
            "gpu" => "get_gpu_temp",
            _ => return Err(anyhow!("Invalid sensor")),
        };
        
        let result = self.acpi_call(cmd, None, None)?;
        
        // Check for error values
        if result == "0xffffffff" {
            return Ok(0); // Return 0 for unavailable sensors
        }
        
        // Clean the result: remove 0x prefix, null bytes, and any non-hex characters
        let clean_result = result
            .trim_start_matches("0x")
            .trim_end_matches('\0')
            .trim()
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect::<String>();
        
        // Parse hex directly
        let temp = u32::from_str_radix(&clean_result, 16)?;
        Ok(temp)
    }
}
