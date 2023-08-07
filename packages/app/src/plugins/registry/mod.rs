use tauri::plugin::{self, TauriPlugin};



pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
  plugin::Builder::new("registry")
    .build()
}
