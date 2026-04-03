import { defineConfig } from "vite";
import vue from '@vitejs/plugin-vue';
import vuetify from 'vite-plugin-vuetify';
import { fileURLToPath } from 'url';
import packageJson from './package.json';

export default defineConfig({
  base: '',
  server: {
    proxy: {
      '/api': 'http://localhost:8080',
      '/api-docs': 'http://localhost:8080',
    }
  },
  plugins: [
    vue(),
    vuetify(),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        quietDeps: true
      },
    }
  },
  define: {
    __PACKAGE_VERSION__: JSON.stringify(packageJson.version)
  },
});
