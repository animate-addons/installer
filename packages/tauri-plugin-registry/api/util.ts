import { platform } from "@tauri-apps/api/os";
import * as invoke from "./invoke";

export const isSupported = (dynamic: boolean = false) =>
  dynamic
  ? platform().then(platform => platform === "win32")
  : invoke.isSupported();
