use tauri::plugin::{TauriPlugin, self};

const JS_INIT_SCRIPT: &str = include_str!("iife.js");

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
  plugin::Builder::new("app")
    .js_init_script(JS_INIT_SCRIPT.into())
    .build()
}
