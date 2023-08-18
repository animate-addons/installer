import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "dist",
    emptyOutDir: true,
    lib: {
      entry: "./api/index.ts",
      fileName: "index",
      formats: ["es"]
    },
    rollupOptions: {
      external: [
        "@tauri-apps/api"
      ]
    }
  }
});
