import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

export default defineConfig({
  base: "/installer/",
  server: {
    port: 5173
  },
  build: {
    emptyOutDir: true,
    outDir: "dist"
  },
  plugins: [solid()],
})
