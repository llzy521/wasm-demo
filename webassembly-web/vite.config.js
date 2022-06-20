import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasmPack from "vite-plugin-rsw";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(),wasmPack()],
});
