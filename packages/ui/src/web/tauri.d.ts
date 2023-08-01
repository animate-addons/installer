declare global {
  interface Window {
    TAURI_PLATFORM: string;
    TAURI_ARCH: string;
    TAURI_FAMILY: string;
    TAURI_PLATFORM_VERSION: string;
    TAURI_PLATFORM_TYPE: string;
    TAURI_DEBUG: boolean;
  }
}

export {}
