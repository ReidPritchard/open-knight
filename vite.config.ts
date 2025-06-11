import UnpluginTypia from "@ryoppippi/unplugin-typia/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;
const isHistoire = process.env.HISTOIRE;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [vue(), UnpluginTypia({})],

	assetsInclude: ["**/*.pgn"],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: isHistoire ? true : false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: isHistoire ? 6006 : 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: "ws",
					host,
					port: 1421,
				}
			: undefined,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));
