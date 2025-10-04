// Dell G Series Controller Core Library
// Provides USB LED control and ACPI power management

pub mod acpi;
pub mod elc;
pub mod elc_constants;
pub mod hid_report;
pub mod keyboard;

// Re-export commonly used types
pub use acpi::{AcpiController, LaptopModel};
pub use keyboard::KeyboardController;

/// Initialize logging for the library
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}
