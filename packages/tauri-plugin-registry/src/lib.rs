use cfg_if::cfg_if;
use tauri::{Runtime, plugin::{Builder, TauriPlugin}};


cfg_if! {
  if #[cfg(target_os = "windows")] {
    use tauri::generate_handler;

    mod handler;
    mod registry;
    
    pub fn init<R: Runtime>() -> TauriPlugin<R> {
      Builder::new("registry")
      .invoke_handler(
        generate_handler![
          handler::is_supported,
          handler::open
          ]
        )
        .build()
    }
  } else {
    pub fn init<R: Runtime>() -> TauriPlugin<R> {
      Builder::new("registry")
        .build()
    }
  }
}
