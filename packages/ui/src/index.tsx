/* @refresh reload */
import { render } from "solid-js/web"
import { App } from "./App"
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";


render(() => <App />, document.getElementById("root")!);

appWindow.setTitle("Animate Addons Installer");
appWindow.center();
appWindow.setSize(new LogicalSize(800, 600));

appWindow.show();
