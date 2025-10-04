// ACPI control for power and fan management
// Translated from the ACPI parts of main.py

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;

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

pub struct AcpiController {
    acpi_cmd_template: String,
    acpi_call_dict: HashMap<String, Vec<String>>,
    pub power_modes: HashMap<String, String>,
    pub model: LaptopModel,
    elevated_shell: Mutex<Option<std::process::Child>>,
    is_root: bool,
}

impl AcpiController {
    pub fn new() -> Result<Self> {
        let mut controller = Self {
            acpi_cmd_template: String::new(),
            acpi_call_dict: HashMap::new(),
            power_modes: Self::default_power_modes(),
            model: LaptopModel::Unknown,
            elevated_shell: Mutex::new(None),
            is_root: false,
        };

        controller.setup_acpi_calls();
        controller.detect_model()?;

        // Initialize elevated shell immediately like Python does
        // This will prompt for authorization once when the app starts
        if let Err(_) = controller.init_elevated_shell() {
            // Fall back to individual pkexec calls
        }

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

    fn init_elevated_shell(&mut self) -> Result<()> {
        
        // Create a shell subprocess (root needed for power related functions)
        match Command::new("bash")
            .arg("--noprofile")
            .arg("--norc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                // Wait for shell prompt
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                // Send initial commands like Python does
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(b" export HISTFILE=/dev/null; history -c\n")?;
                    stdin.flush()?;
                    
                    // Elevate privileges (pkexec is needed)
                    stdin.write_all(b"pkexec bash --noprofile --norc\n")?;
                    stdin.flush()?;
                    
                    // Wait for pkexec to complete
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    
                    // Send more commands
                    stdin.write_all(b" export HISTFILE=/dev/null; history -c\n")?;
                    stdin.flush()?;
                    
                    // Check if root
                    stdin.write_all(b"whoami\n")?;
                    stdin.flush()?;
                    
                    // Read response
                    if let Some(ref mut stdout) = child.stdout {
                        let mut reader = BufReader::new(stdout);
                        let mut line = String::new();
                        
                        // Read whoami output
                        if reader.read_line(&mut line).is_ok() {
                            if line.contains("root") {
                                self.is_root = true;
                                
                                // Store the elevated shell
                                *self.elevated_shell.lock().unwrap() = Some(child);
                                return Ok(());
                            }
                        }
                    }
                }
                
                // Fall back to individual pkexec calls
                self.is_root = false;
                Ok(())
            }
            Err(_) => {
                // Fall back to individual pkexec calls
                self.is_root = false;
                Ok(())
            }
        }
    }



    fn detect_model(&mut self) -> Result<()> {
        // First try DMI detection (works without root) - same as Python version
        if let Ok(output) = Command::new("cat")
            .arg("/sys/class/dmi/id/product_name")
            .output()
        {
            let product = String::from_utf8_lossy(&output.stdout).to_lowercase();
            
            if product.contains("g15") && product.contains("5530") {
                self.model = LaptopModel::G15_5530;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            } else if product.contains("g15") && product.contains("5520") {
                self.model = LaptopModel::G15_5520;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            } else if product.contains("g15") && product.contains("5525") {
                self.model = LaptopModel::G15_5525;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMW3.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            } else if product.contains("g15") && product.contains("5515") {
                self.model = LaptopModel::G15_5515;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMW3.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            } else if product.contains("g15") && product.contains("5511") {
                self.model = LaptopModel::G15_5511;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            } else if product.contains("g16") && product.contains("7630") {
                self.model = LaptopModel::G16_7630;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            } else if product.contains("g16") && product.contains("7620") {
                self.model = LaptopModel::G16_7620;
                self.acpi_cmd_template = 
                    "echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                        .to_string();
                self.apply_model_patch();
                return Ok(());
            }
        }
        
        self.acpi_cmd_template = 
            "echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                .to_string();
        
        match self.acpi_call("get_laptop_model", None, None) {
            Ok(model_str) => {
                let model = model_str.trim();
                self.model = match model {
                    "0x0" => LaptopModel::G15_5530,
                    "0x12c0" => LaptopModel::G15_5520,
                    "0xc80" => LaptopModel::G15_5511,
                    _ => LaptopModel::Unknown,
                };
                
                if self.model != LaptopModel::Unknown {
                    self.apply_model_patch();
                    return Ok(());
                }
            }
            Err(_) => {}
        }
        
        // Try AMD models
        self.acpi_cmd_template = 
            "echo \"\\_SB.AMW3.WMAX 0 {} {{{}, {}, {}, 0x00}}\" | tee /proc/acpi/call; cat /proc/acpi/call"
                .to_string();
        
        match self.acpi_call("get_laptop_model", None, None) {
            Ok(model_str) => {
                let model = model_str.trim();
                self.model = match model {
                    "0x12c0" => LaptopModel::G15_5525,
                    "0xc80" => LaptopModel::G15_5515,
                    _ => LaptopModel::Unknown,
                };
                
                if self.model != LaptopModel::Unknown {
                    self.apply_model_patch();
                }
            }
            Err(_) => {}
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

        // Use the same format as the original Python project
        let cmd_str = match args.len() {
            4 => format!("echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" > /proc/acpi/call", args[0], args[1], args[2], args[3]),
            3 => {
                let arg1 = arg1.unwrap_or("0x00");
                format!("echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" > /proc/acpi/call", args[0], args[1], args[2], arg1)
            }
            2 => {
                let arg1 = arg1.unwrap_or("0x00");
                let arg2 = arg2.unwrap_or("0x00");
                format!("echo \"\\_SB.AMWW.WMAX 0 {} {{{}, {}, {}, 0x00}}\" > /proc/acpi/call", args[0], args[1], arg1, arg2)
            }
            _ => return Err(anyhow!("Invalid ACPI call format")),
        };

        // For now, use individual pkexec calls since the persistent shell has issues
        let output = Command::new("pkexec")
            .arg("sh")
            .arg("-c")
            .arg(&format!("{}; cat /proc/acpi/call", cmd_str))
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("ACPI call failed"));
        }

        let result = String::from_utf8_lossy(&output.stdout);

        let lines: Vec<&str> = result.lines().collect();
        
        // Get the last line which contains the result
        if let Some(last_line) = lines.last() {
            let trimmed = last_line.trim().trim_end_matches('%').trim_matches('\'');
            Ok(trimmed.to_string())
        } else {
            Err(anyhow!("No output from ACPI call"))
        }
    }

    pub fn set_power_mode(&mut self, mode: &str) -> Result<()> {
        let mode_value = self
            .power_modes
            .get(mode)
            .ok_or_else(|| anyhow!("Unknown power mode: {}", mode))?
            .clone();
        
        self.acpi_call("set_power_mode", Some(&mode_value), None)?;
        Ok(())
    }

    #[allow(unused)]
    pub fn get_power_mode(&mut self) -> Result<String> {
        self.acpi_call("get_power_mode", None, None)
    }

    pub fn set_fan_boost(&mut self, fan_id: u8, boost: u8) -> Result<()> {
        // Use the working command format for G15 5530
        let cmd = format!("echo \"\\_SB.AMWW.WMAX 0 0x15 {{0x02, 0x{}, 0x{:02X}, 0x00}}\" > /proc/acpi/call", 
                         if fan_id == 1 { "32" } else { "33" }, boost);

        let output = Command::new("pkexec")
            .arg("sh")
            .arg("-c")
            .arg(&format!("{}; cat /proc/acpi/call", cmd))
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!("Fan boost command failed"))
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
