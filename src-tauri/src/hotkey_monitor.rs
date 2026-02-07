use evdev::{Device, InputEventKind, Key};
use std::path::PathBuf;
use std::sync::mpsc;

pub struct HotkeyMonitor {
    rx: mpsc::Receiver<()>,
}

impl HotkeyMonitor {
    pub fn new(target_key: Key) -> Result<Self, String> {
        let keyboards = find_keyboards()?;
        if keyboards.is_empty() {
            return Err("Nenhum teclado encontrado em /dev/input/".to_string());
        }

        let (tx, rx) = mpsc::channel();

        for path in keyboards {
            let tx = tx.clone();
            let path_str = path.display().to_string();

            std::thread::spawn(move || {
                let mut device = match Device::open(&path) {
                    Ok(d) => d,
                    Err(e) => {
                        log::warn!("Não foi possível abrir {}: {}", path_str, e);
                        return;
                    }
                };

                log::info!(
                    "Monitorando teclado '{}' para hotkey em {}",
                    device.name().unwrap_or("desconhecido"),
                    path_str
                );

                loop {
                    match device.fetch_events() {
                        Ok(events) => {
                            for event in events {
                                if let InputEventKind::Key(key) = event.kind() {
                                    // value 1 = key press
                                    if key == target_key && event.value() == 1 {
                                        log::info!("Hotkey {:?} detectada em {}", target_key, path_str);
                                        let _ = tx.send(());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Erro ao ler eventos de {}: {}", path_str, e);
                            break;
                        }
                    }
                }
            });
        }

        Ok(HotkeyMonitor { rx })
    }

    /// Non-blocking check for hotkey events
    pub fn try_recv(&self) -> bool {
        self.rx.try_recv().is_ok()
    }
}

/// Find all keyboard devices in /dev/input/
fn find_keyboards() -> Result<Vec<PathBuf>, String> {
    let mut keyboards = Vec::new();

    let entries = std::fs::read_dir("/dev/input/")
        .map_err(|e| format!("Não foi possível ler /dev/input/: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if !name.starts_with("event") {
            continue;
        }

        let device = match Device::open(&path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        // Check if this device is a keyboard (has alphabetic keys + F9)
        if let Some(keys) = device.supported_keys() {
            let is_keyboard = keys.contains(Key::KEY_A)
                && keys.contains(Key::KEY_Z)
                && keys.contains(Key::KEY_ENTER);
            let has_target = keys.contains(Key::KEY_F9);

            if is_keyboard && has_target {
                log::info!(
                    "Teclado encontrado: {} - '{}'",
                    path.display(),
                    device.name().unwrap_or("desconhecido")
                );
                keyboards.push(path);
            }
        }
    }

    Ok(keyboards)
}
