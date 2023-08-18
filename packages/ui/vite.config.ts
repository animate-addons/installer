import path from 'path'
import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

const platform = 
  process.platform === "win32" ? "windows"
  : process.platform === "darwin" ? "macos"
  : process.platform === "linux" ? "linux"
  : undefined;
const arch =
  process.arch === "x64" ? "x86_64"
  : process.arch === "ia32" ? "x86"
  : process.arch === "arm64" ? "arm_x64"
  : undefined;

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
    outDir: "dist",
    emptyOutDir: true,
  },
  plugins: [solid()],
})
