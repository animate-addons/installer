#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod registry;

fn main() {
  let context = tauri::generate_context!();

  tauri::Builder::default()
    .plugin(registry::plugin::init())
    .run(context)
    .expect("Failed to start the installer!");
}
