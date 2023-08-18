/* @refresh reload */
import { render } from "solid-js/web"
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
import { Component, lazy } from "solid-js";


// if(window.__INSTALLER__?.active) {
//   render(() => <App />, document.getElementById("root")!);

//   window.addEventListener("DOMContentLoaded", () => appWindow.show());
// } else {
//   window.location.replace("https://github.com/animate-addons/installer/");
// }

const App = lazy(
  () => 
    window.__INSTALLER__?.active
    ? import("./app/App")
    : import("./website/App")
)

render(() => <App />, document.getElementById("root")!);
