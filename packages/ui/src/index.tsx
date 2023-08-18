/* @refresh reload */
import { render } from "solid-js/web"
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
import { App } from "./installer/App";
import { isSupported } from "tauri-plugin-registry";

import "./index.css";


if(window.__INSTALLER__?.active) {
  render(() => <App />, document.getElementById("root")!);

  window.addEventListener("DOMContentLoaded", () => appWindow.show());
} else window.location.replace("https://github.com/animate-addons/installer/");
