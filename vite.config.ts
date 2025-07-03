import UnpluginTypia from "@ryoppippi/unplugin-typia/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";

const dev = process.env.NODE_ENV === "development";
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(() => ({
	plugins: [vue(), UnpluginTypia({})],

	assetsInclude: ["**/*.pgn"],

	esbuild: {
		logOverride: {
			// Typia logs a lot of warning for a number of the json validators it generates.
			// This is a workaround to silence them.
			"suspicious-logical-operator": dev
				? ("warning" as const)
				: ("silent" as const),
		},
	},

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
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

	build: {
		manifest: true,
		// TODO: Don't bundle stories on vite build
	},
}));
