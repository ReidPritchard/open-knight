import { defineConfig } from "histoire";
import { HstVue } from "@histoire/plugin-vue";

export default defineConfig({
	plugins: [HstVue()],
	storyIgnored: ["**/node_modules/**", "**/dist/**", "**/src-tauri/**"],
	storyMatch: ["**/*.story.vue"],
	setupFile: "/src/histoire.setup.ts",
	tree: {
		file: "path",
	},
	collectMaxThreads: 4,
});
