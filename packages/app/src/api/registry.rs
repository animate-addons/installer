#![cfg(target_os = "windows")]

use tauri::plugin;

const PLUGIN_NAME: &str = "installer:registry";

pub fn init<R: tauri::Runtime>() -> plugin::TauriPlugin<R> {
  plugin::Builder::new(PLUGIN_NAME)
    .build()
}
