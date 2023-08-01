interface InstallerGlobal {
  active: boolean;
} 
declare global {
  interface Window {
    __INSTALLER__: InstallerGlobal | undefined;
  }
}

export {}
