import { defineSetupVue3 } from "@histoire/plugin-vue";
import { createPinia } from "pinia";
import {
	GlobalStoreKey,
	ImportExportServiceKey,
	createMockGlobalStore,
	createMockImportExportService,
} from "./composables/useInjection";

import ComponentWrapper from "./histoire/ComponentWrapper.vue";

import "./style.css";

export const setupVue3 = defineSetupVue3(({ app, addWrapper }) => {
	const pinia = createPinia();
	app.use(pinia);

	// Provide mock implementations for all stories using app.provide
	app.provide(GlobalStoreKey, createMockGlobalStore());
	app.provide(ImportExportServiceKey, createMockImportExportService());

	addWrapper(ComponentWrapper);

	console.log("Histoire setup complete with mock providers");
});
