use tauri::plugin::{self, TauriPlugin};

pub mod util;
mod command;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
  plugin::Builder::new("registry")
    .invoke_handler(tauri::generate_handler![
      command::get_tree
    ])
    .build()
}
