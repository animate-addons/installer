use tauri::plugin::{TauriPlugin, self};

const JS_INIT_SCRIPT: &str = include_str!("js_init_script.js");

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
  plugin::Builder::new("installer")
    .js_init_script(JS_INIT_SCRIPT.into())
    .build()
}
