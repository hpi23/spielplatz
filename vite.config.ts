import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],
    build: {
        rollupOptions: {
            output: {
                manualChunks: (id: any) => {
                    if (id.includes('node_modules')) {
                        if (id.includes('@smui') || id.includes('@material')) {
                            return 'vendor_mui'
                        }
                        return 'vendor'
                    }
                },
            },
        },
    },
})
