import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  root: './client',
  plugins: [svelte()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
      '/judge': {
        target: 'http://localhost:2358',
        rewrite: (path) => path.replace(/^\/judge/, ''),
      },
    },
  },
});
