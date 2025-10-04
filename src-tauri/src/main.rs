// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Set environment variables to fix GBM buffer issues on Linux
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    std::env::set_var("GDK_BACKEND", "x11");

    dell_gui_tauri_lib::run()
}
