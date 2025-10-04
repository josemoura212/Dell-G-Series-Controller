use dell_core::{AcpiController, KeyboardController};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, State};

// Application state
pub struct AppState {
    keyboard: Arc<Mutex<Option<KeyboardController>>>,
    acpi: Arc<Mutex<Option<AcpiController>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    model: String,
    keyboard_supported: bool,
    power_supported: bool,
    power_modes: Vec<String>,
    fan_control_limited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    fan1_rpm: u32,
    fan2_rpm: u32,
    cpu_temp: u32,
    gpu_temp: u32,
}

// System setup commands
#[tauri::command]
async fn check_permissions() -> Result<String, String> {
    // Check if udev rules exist
    let udev_rules = Path::new("/etc/udev/rules.d/99-dell-g-series.rules");
    let polkit_rules = Path::new("/etc/polkit-1/rules.d/50-dell-acpi-nopasswd.rules");

    let mut missing = Vec::new();

    if !udev_rules.exists() {
        missing.push("udev");
    }
    if !polkit_rules.exists() {
        missing.push("polkit");
    }

    if missing.is_empty() {
        Ok("configured".to_string())
    } else {
        Ok(format!("missing:{}", missing.join(",")))
    }
}

#[tauri::command]
async fn run_setup_script() -> Result<String, String> {
    // Get the executable directory and look for setup-acpi.sh in the parent directory
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Erro ao obter caminho do executável: {}", e))?;
    let exe_dir = exe_path.parent().ok_or("Caminho do executável inválido")?;
    let script_path = exe_dir.parent().unwrap_or(exe_dir).join("setup-acpi.sh");

    if !script_path.exists() {
        return Err(format!(
            "Script de configuração não encontrado em: {:?}",
            script_path
        ));
    }

    let output = Command::new("pkexec")
        .arg("bash")
        .arg(&script_path)
        .output()
        .map_err(|e| format!("Erro ao executar script: {}", e))?;

    if output.status.success() {
        Ok("✓ Configuração concluída! Reinicie o sistema para aplicar as mudanças.".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Erro na configuração: {}", error))
    }
}

#[tauri::command]
fn check_usb_devices(state: State<AppState>) -> Result<Vec<String>, String> {
    let mut devices = Vec::new();

    // Check if keyboard controller is available
    let keyboard = state.keyboard.lock().unwrap();
    if keyboard.is_some() {
        devices.push("187c:0550/0551".to_string());
    }

    if devices.is_empty() {
        Err(
            "Nenhum dispositivo Dell compatível encontrado. Verifique as permissões USB."
                .to_string(),
        )
    } else {
        Ok(devices)
    }
}

// Initialize device controllers
#[tauri::command]
fn init_device(state: State<AppState>) -> Result<DeviceInfo, String> {
    let keyboard = state.keyboard.lock().unwrap();
    let acpi = state.acpi.lock().unwrap();

    let keyboard_supported = keyboard.is_some();
    let power_supported = acpi.is_some();

    let (model, power_modes, fan_control_limited) = match acpi.as_ref() {
        Some(acpi_controller) => {
            let modes: Vec<String> = acpi_controller.power_modes.keys().cloned().collect();
            let model_str = acpi_controller.model.as_str().to_string();
            // Fan control is NOT limited on G15 5530 - our tests show it works!
            // Only limit on specific models that are known to not support manual control
            let limited = matches!(
                acpi_controller.model,
                dell_core::acpi::LaptopModel::G15_5515
            );
            (model_str, modes, limited)
        }
        None => ("Unknown".to_string(), Vec::new(), false),
    };

    Ok(DeviceInfo {
        model,
        keyboard_supported,
        power_supported,
        power_modes,
        fan_control_limited,
    })
}

// Keyboard LED commands
#[tauri::command]
fn set_static_color(
    state: State<AppState>,
    red: u8,
    green: u8,
    blue: u8,
) -> Result<String, String> {
    let keyboard = state.keyboard.lock().unwrap();
    if let Some(kb) = keyboard.as_ref() {
        kb.set_static(red, green, blue).map_err(|e| e.to_string())?;
        Ok(format!("✓ Cor aplicada: RGB({}, {}, {})", red, green, blue))
    } else {
        Err("Keyboard not available".to_string())
    }
}

#[tauri::command]
fn set_morph(
    state: State<AppState>,
    red: u8,
    green: u8,
    blue: u8,
    duration: u16,
) -> Result<String, String> {
    let keyboard = state.keyboard.lock().unwrap();
    if let Some(kb) = keyboard.as_ref() {
        kb.set_morph(red, green, blue, duration)
            .map_err(|e| e.to_string())?;
        Ok("✓ Modo morph aplicado".to_string())
    } else {
        Err("Keyboard not available".to_string())
    }
}

#[tauri::command]
fn set_color_and_morph(
    state: State<AppState>,
    red_static: u8,
    green_static: u8,
    blue_static: u8,
    red_morph: u8,
    green_morph: u8,
    blue_morph: u8,
    duration: u16,
) -> Result<String, String> {
    let keyboard = state.keyboard.lock().unwrap();
    if let Some(kb) = keyboard.as_ref() {
        kb.set_color_and_morph(
            red_static,
            green_static,
            blue_static,
            red_morph,
            green_morph,
            blue_morph,
            duration,
        )
        .map_err(|e| e.to_string())?;
        Ok("✓ Modo color+morph aplicado".to_string())
    } else {
        Err("Keyboard not available".to_string())
    }
}

#[tauri::command]
fn turn_off_leds(state: State<AppState>) -> Result<String, String> {
    let keyboard = state.keyboard.lock().unwrap();
    if let Some(kb) = keyboard.as_ref() {
        kb.remove_all_animations().map_err(|e| e.to_string())?;
        Ok("✓ LEDs desligados".to_string())
    } else {
        Err("Keyboard not available".to_string())
    }
}

#[tauri::command]
fn set_dim(state: State<AppState>, level: u8) -> Result<String, String> {
    let keyboard = state.keyboard.lock().unwrap();
    if let Some(kb) = keyboard.as_ref() {
        kb.set_dim(level).map_err(|e| e.to_string())?;
        Ok(format!("✓ Brilho ajustado: {}%", level))
    } else {
        Err("Keyboard not available".to_string())
    }
}

// Power management commands
#[tauri::command]
fn set_power_mode(state: State<AppState>, mode: String) -> Result<String, String> {
    let mut acpi = state.acpi.lock().unwrap();
    if let Some(acpi_controller) = acpi.as_mut() {
        acpi_controller
            .set_power_mode(&mode)
            .map_err(|e| e.to_string())?;

        // If Performance mode is selected, set fans to 100%
        if mode == "USTT_Performance" {
            std::thread::sleep(std::time::Duration::from_millis(200));

            let mut success_count = 0;
            let mut errors = Vec::new();

            // Set both fans to 100% (0xFF)
            match acpi_controller.set_fan_boost(1, 0xFF) {
                Ok(_) => success_count += 1,
                Err(e) => errors.push(format!("CPU fan: {}", e)),
            }

            match acpi_controller.set_fan_boost(2, 0xFF) {
                Ok(_) => success_count += 1,
                Err(e) => errors.push(format!("GPU fan: {}", e)),
            }

            if success_count > 0 {
                return Ok(format!(
                    "✓ Modo Performance ativado - Ventiladores em 100% ({}/2 sucesso)",
                    success_count
                ));
            } else {
                return Ok(format!(
                    "✓ Modo Performance ativado (ventiladores não suportados: {})",
                    errors.join(", ")
                ));
            }
        }

        Ok(format!("✓ Modo de energia: {}", mode))
    } else {
        Err("ACPI not available".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanBoostParams {
    pub cpu_rpm: u8,
    pub gpu_rpm: u8,
}

#[tauri::command]
fn set_fan_boost(state: State<AppState>, params: FanBoostParams) -> Result<String, String> {
    let mut acpi = state.acpi.lock().unwrap();
    if let Some(acpi_controller) = acpi.as_mut() {
        // Ensure we're in Manual mode for fan control
        if let Err(_) = acpi_controller.set_power_mode("Manual") {
            // Log warning but continue
        }

        // Small delay to let the mode change take effect
        std::thread::sleep(std::time::Duration::from_millis(200));

        // Convert percentage (0-100) to ACPI scale (0-255)
        let cpu_acpi_value = ((params.cpu_rpm as f32 / 100.0) * 255.0) as u8;
        let gpu_acpi_value = ((params.gpu_rpm as f32 / 100.0) * 255.0) as u8;

        let mut success_count = 0;
        let mut errors = Vec::new();

        // Set CPU fan (fan 1)
        match acpi_controller.set_fan_boost(1, cpu_acpi_value) {
            Ok(_) => success_count += 1,
            Err(e) => errors.push(format!("CPU fan: {}", e)),
        }

        // Set GPU fan (fan 2)
        match acpi_controller.set_fan_boost(2, gpu_acpi_value) {
            Ok(_) => success_count += 1,
            Err(e) => errors.push(format!("GPU fan: {}", e)),
        }

        if success_count > 0 {
            Ok(format!(
                "✓ Ventiladores ajustados: CPU {}%, GPU {}% ({}/2 sucesso)",
                params.cpu_rpm, params.gpu_rpm, success_count
            ))
        } else {
            Err(format!(
                "❌ Controle manual de fans não disponível neste sistema. Use os presets ou verifique as permissões ACPI. Erros: {}",
                errors.join(", ")
            ))
        }
    } else {
        Err("ACPI not available".to_string())
    }
}

#[tauri::command]
fn set_turbo_mode(state: State<AppState>) -> Result<String, String> {
    let mut acpi = state.acpi.lock().unwrap();
    if let Some(acpi_controller) = acpi.as_mut() {
        // Set fans to maximum (100%)
        let mut success_count = 0;
        let mut errors = Vec::new();

        // Set both fans to 100% (0xFF)
        match acpi_controller.set_fan_boost(1, 0xFF) {
            Ok(_) => success_count += 1,
            Err(e) => errors.push(format!("CPU fan: {}", e)),
        }

        match acpi_controller.set_fan_boost(2, 0xFF) {
            Ok(_) => success_count += 1,
            Err(e) => errors.push(format!("GPU fan: {}", e)),
        }

        if success_count > 0 {
            Ok(format!(
                "🚀 MODO TURBO ATIVADO - Ventiladores em 100% ({}/2 sucesso)",
                success_count
            ))
        } else {
            Err(format!(
                "❌ Modo turbo não disponível: {}",
                errors.join(", ")
            ))
        }
    } else {
        Err("ACPI not available".to_string())
    }
}

#[tauri::command]
fn get_sensors(state: State<AppState>) -> Result<SensorData, String> {
    let mut acpi = state.acpi.lock().unwrap();
    if let Some(acpi_controller) = acpi.as_mut() {
        let fan1_rpm = acpi_controller.get_fan_rpm(1).unwrap_or(0);
        let fan2_rpm = acpi_controller.get_fan_rpm(2).unwrap_or(0);
        let cpu_temp = acpi_controller.get_temp("cpu").unwrap_or(0);
        let gpu_temp = acpi_controller.get_temp("gpu").unwrap_or(0);

        Ok(SensorData {
            fan1_rpm,
            fan2_rpm,
            cpu_temp,
            gpu_temp,
        })
    } else {
        Err("ACPI not available".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    // Initialize controllers
    let keyboard = Arc::new(Mutex::new(KeyboardController::new(false).ok()));

    // Try to create ACPI controller and log any errors
    let acpi_result = AcpiController::new();
    let acpi = Arc::new(Mutex::new(match acpi_result {
        Ok(controller) => Some(controller),
        Err(_) => None,
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .manage(AppState { keyboard, acpi })
        .invoke_handler(tauri::generate_handler![
            check_permissions,
            run_setup_script,
            check_usb_devices,
            init_device,
            set_static_color,
            set_morph,
            set_color_and_morph,
            turn_off_leds,
            set_dim,
            set_power_mode,
            set_fan_boost,
            set_turbo_mode,
            get_sensors,
        ])
        .setup(|app| {
            // Register global shortcut for turbo mode
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["f12"])?
                        .with_handler(|app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::empty(), Code::F12) {
                                    let app_handle = app.clone();
                                    tauri::async_runtime::spawn(async move {
                                        // Call the turbo mode command directly
                                        let state = app_handle.state::<AppState>();
                                        if let Err(e) = set_turbo_mode(state) {
                                            eprintln!("Failed to set turbo mode: {:?}", e);
                                        }
                                    });
                                }
                            }
                        })
                        .build(),
                )?;
            }

            // Create system tray
            let show = MenuItemBuilder::with_id("show", "Mostrar").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Sair").build(app)?;
            let menu = MenuBuilder::new(app).item(&show).item(&quit).build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Don't close, just hide
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
