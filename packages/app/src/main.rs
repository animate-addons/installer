#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod api;

fn main() {
  let context = tauri::generate_context!();
  let builder = tauri::Builder::default()
    .plugin(api::plugin::init())
    .plugin(api::registry::init());
  builder
    .run(context)
    .expect("Failed to start the installer!");
}
