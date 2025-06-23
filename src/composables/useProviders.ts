import { App, provide } from "vue";
import {
	GlobalStoreKey,
	ImportExportServiceKey,
	createMockGlobalStore,
	createMockImportExportService,
	type IGlobalStore,
	type IImportExportService,
} from "./useInjection";

// Helper to build real implementations for stores and services
async function buildRealImplementations() {
	const { useGlobalStore } = await import("../stores");
	const ImportExportService = await import("../services/ImportExportService");
	const globalStore = useGlobalStore();

	const globalStoreImpl: IGlobalStore = {
		importPGNGames: (pgn: string) => globalStore.importPGNGames(pgn),
		updateGameProperty: (gameId, property, value) =>
			globalStore.updateGameProperty(gameId, property, value),
		fetchExplorerGames: async () => {
			const result = await globalStore.fetchExplorerGames();
			return Array.isArray(result) ? result : globalStore.explorerGames;
		},
		explorerGames: globalStore.explorerGames,
	};

	const importExportServiceImpl: IImportExportService = {
		validatePGNFormat: ImportExportService.validatePGNFormat,
		importPGNGames: ImportExportService.importPGNGames,
		fetchExplorerGames: async () => {
			const result = await ImportExportService.fetchExplorerGames();
			return result && result.success && Array.isArray(result.data)
				? result.data
				: [];
		},
		searchGames: ImportExportService.searchGames,
		filterGames: ImportExportService.filterGames,
		sortGames: ImportExportService.sortGames,
	};

	return { globalStoreImpl, importExportServiceImpl };
}

/**
 * Provides the real implementations of stores and services
 * Call this in your main app setup
 */
export const useProviders = async () => {
	const { globalStoreImpl, importExportServiceImpl } =
		await buildRealImplementations();
	provide(GlobalStoreKey, globalStoreImpl);
	provide(ImportExportServiceKey, importExportServiceImpl);
};

/**
 * Provides real implementations using app.provide
 * Call this in your main app setup with the app instance
 */
export const useAppProviders = async (app: App) => {
	const { globalStoreImpl, importExportServiceImpl } =
		await buildRealImplementations();
	app.provide(GlobalStoreKey, globalStoreImpl);
	app.provide(ImportExportServiceKey, importExportServiceImpl);
};

/**
 * Provides mock implementations for testing/stories
 * Call this in your test setup or Histoire setup
 */
export const useMockProviders = () => {
	const mockGlobalStore = createMockGlobalStore();
	const mockImportExportService = createMockImportExportService();
	provide(GlobalStoreKey, mockGlobalStore);
	provide(ImportExportServiceKey, mockImportExportService);
};
