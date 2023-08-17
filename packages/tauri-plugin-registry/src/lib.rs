use tauri::{Runtime, plugin::{Builder, TauriPlugin}, generate_handler};

mod handler;
mod registry;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("registry")
    .invoke_handler(
      generate_handler![
        handler::open
      ]
    )
    .build()
}
