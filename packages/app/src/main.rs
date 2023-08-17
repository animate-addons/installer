#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#![feature(iterator_try_collect)]


mod app;
mod plugins;

fn main() {
  let context = tauri::generate_context!();
  let builder = tauri::Builder::default()
    .plugin(app::init())
    .plugin(tauri_plugin_registry::init());
  builder
    .run(context)
    .expect("Failed to start the installer!");
}
