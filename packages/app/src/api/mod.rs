pub mod registry;

pub mod plugin {
  use tauri::plugin;

  const JS_INIT_SCRIPT: &str = r#"
  Object.defineProperty(
    window,
    "__INSTALLER__",
    {
      configurable: false,
      enumerable: true,
      get: () => {
        return {
          active: true,
          production: /^https:\/\/.*\.github.io\/.*$/.test(window.location.href),
          development: /^https?:\/\/localhost:\d+(\/.*)?$/.test(window.location.href)
        };
      }
    }
  );
  "#;

  pub fn init<R: tauri::Runtime>() -> plugin::TauriPlugin<R> {
    plugin::Builder::new("installer:init")
      .js_init_script(JS_INIT_SCRIPT.to_string())
      .build()
  }
}
