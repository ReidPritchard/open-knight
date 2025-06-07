import js from "@eslint/js";
import pluginVue from "eslint-plugin-vue";
import tseslint from "typescript-eslint";

export default [
	// Base JavaScript rules
	js.configs.recommended,

	// TypeScript rules
	...tseslint.configs.recommended,

	// Vue rules
	...pluginVue.configs["flat/recommended"],

	{
		files: ["**/*.{js,mjs,cjs,ts,vue}"],
		languageOptions: {
			ecmaVersion: "latest",
			sourceType: "module",
		},
		rules: {
			// Disable some overly strict rules for better Vue compatibility
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
					caughtErrorsIgnorePattern: "^_",
				},
			],
			"@typescript-eslint/no-explicit-any": "warn",
			"@typescript-eslint/ban-ts-comment": "warn",
			"@typescript-eslint/no-empty-object-type": "warn",
			"no-undef": "off", // TypeScript handles this
			"no-console": "warn",

			// Vue-specific rules
			"vue/multi-word-component-names": "off",
			"vue/no-v-html": "warn",
			"vue/require-default-prop": "off",
			"vue/require-explicit-emits": "error",
			"vue/component-definition-name-casing": ["error", "PascalCase"],
			"vue/component-name-in-template-casing": ["error", "PascalCase"],
			"vue/define-macros-order": [
				"error",
				{
					order: ["defineProps", "defineEmits"],
				},
			],

			// Disable formatting rules that conflict with Prettier
			"vue/html-indent": "off",
			"vue/html-self-closing": "off",
			"vue/multiline-html-element-content-newline": "off",
			"vue/singleline-html-element-content-newline": "off",
			"vue/attributes-order": "off",
			"vue/block-order": "off", // Let developers organize as they prefer
		},
	},

	{
		files: ["**/*.vue"],
		languageOptions: {
			parser: pluginVue.parser,
			parserOptions: {
				parser: tseslint.parser,
				extraFileExtensions: [".vue"],
				ecmaVersion: "latest",
				sourceType: "module",
			},
		},
		rules: {
			// Additional Vue-specific overrides
		},
	},

	{
		files: ["**/*.ts", "**/*.tsx"],
		languageOptions: {
			parser: tseslint.parser,
			parserOptions: {
				project: "./tsconfig.json",
			},
		},
	},

	{
		ignores: [
			"dist/**",
			"node_modules/**",
			"src-tauri/**",
			"vite-env.d.ts",
			"*.config.js",
			"*.config.ts",
		],
	},
];
