import path from 'path'
import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

export default defineConfig({
  optimizeDeps: {
    force: true
  },
  resolve: {
    preserveSymlinks: true
  },
  base: "/installer/",
  server: {
    port: 5173
  },
  build: {
    emptyOutDir: true,
    outDir: "dist"
  },
  envPrefix: [
    "VITE_",
    "TAURI_"
  ],
  plugins: [solid()],
})
