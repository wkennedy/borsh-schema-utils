import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        wasm(),
        topLevelAwait()
    ],
    build: {
        target: 'esnext', // Needed for top-level await support
    },
    optimizeDeps: {
        // This is needed to prevent Vite from trying to process WASM files during dependency optimization
        exclude: ['unborsh-wasm']
    }
})