import { defineConfig } from "histoire";
import { HstVue } from "@histoire/plugin-vue";

export default defineConfig({
	plugins: [HstVue()],
	storyIgnored: ["**/node_modules/**", "**/dist/**", "**/src-tauri/**"],
	storyMatch: ["**/*.story.vue"],
	setupFile: "/src/histoire.setup.ts",
	tree: {
		file: "title",
	},
	collectMaxThreads: 4,
	vite: {
		server: {
			port: 6006,
		},
		define: {
			// Define globals for Node.js compatibility
			global: "globalThis",
		},
		optimizeDeps: {
			// Force optimization of typia to fix exports issue
			include: ["typia"],
			force: true,
		},
		resolve: {
			alias: {
				// Enable template compilation for Vue
				vue: "vue/dist/vue.esm-bundler.js",
				// Provide browser-compatible alternatives for Node.js modules
				util: "util",
			},
		},
	},
	theme: {
		title: "Open Knight",
	},
});
