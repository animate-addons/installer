interface Installer {
  active: boolean;
} 
declare global {
  interface Window {
    __INSTALLER__?: Installer;
  }
}

export {}
