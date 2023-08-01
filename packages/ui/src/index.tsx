/* @refresh reload */
import { render } from "solid-js/web"
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
import { InstallerApp } from "./installer/InstallerApp";


if(window.__INSTALLER__?.active) {
  render(() => <InstallerApp />, document.getElementById("root")!);

  window.addEventListener("DOMContentLoaded", () => appWindow.show());
} else window.location.replace("https://github.com/animate-addons/installer/");
