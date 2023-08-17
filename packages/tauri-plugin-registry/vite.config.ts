import { defineConfig } from "vite";
import dts from "vite-plugin-dts";

export default defineConfig({
  build: {
    outDir: "dist",
    emptyOutDir: true,
    lib: {
      entry: "./api/index.ts",
      fileName: "index",
      formats: ["cjs", "es"]
    },
    rollupOptions: {
      external: [
        "@tauri-apps/api"
      ]
    }
  },
  plugins: [
    dts({
      insertTypesEntry: true,
      exclude: "vite.config.ts"
    })
  ]
});
