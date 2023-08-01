/* @refresh reload */
import { render } from "solid-js/web"
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
import { InstallerApp } from "./installer/InstallerApp";





render(() => <InstallerApp />, document.getElementById("root")!);

appWindow.setTitle("Animate Addons Installer");
appWindow.center();
appWindow.setSize(new LogicalSize(800, 600));

appWindow.show();
