use tauri::plugin;

pub fn init<R: tauri::Runtime>() -> plugin::TauriPlugin<R> {
  plugin::Builder::new("registry")
    .build()
}
