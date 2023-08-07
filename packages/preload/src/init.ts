Object.defineProperty(
  window,
  "__INSTALLER__",
  {
    configurable: false,
    enumerable: true,
    get() {
      return {
        active: true,
      }
    }
  }
);
