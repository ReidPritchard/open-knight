import { App, provide } from "vue";
import {
	GlobalStoreKey,
	ImportExportServiceKey,
	type IGlobalStore,
	type IImportExportService,
} from "./useInjection";

/**
 * Provides the real implementations of stores and services
 * Call this in your main app setup
 */
export const useProviders = async () => {
	// Dynamically import to avoid loading issues in test environments
	const { useGlobalStore } = await import("../stores");
	const ImportExportService = await import("../services/ImportExportService");

	const globalStore = useGlobalStore();

	// Provide the real global store
	const globalStoreImpl: IGlobalStore = {
		importPGNGames: (pgn: string) => globalStore.importPGNGames(pgn),
		// Add other methods as needed
	};

	// Provide the real import/export service
	const importExportServiceImpl: IImportExportService = {
		validatePGNFormat: ImportExportService.validatePGNFormat,
		importPGNGames: ImportExportService.importPGNGames,
		// Add other methods as needed
	};

	provide(GlobalStoreKey, globalStoreImpl);
	provide(ImportExportServiceKey, importExportServiceImpl);
};

/**
 * Provides real implementations using app.provide
 * Call this in your main app setup with the app instance
 */
export const useAppProviders = async (app: App) => {
	// Dynamically import to avoid loading issues in test environments
	const { useGlobalStore } = await import("../stores");
	const ImportExportService = await import("../services/ImportExportService");

	const globalStore = useGlobalStore();

	// Provide the real global store
	const globalStoreImpl: IGlobalStore = {
		importPGNGames: (pgn: string) => globalStore.importPGNGames(pgn),
	};

	// Provide the real import/export service
	const importExportServiceImpl: IImportExportService = {
		validatePGNFormat: ImportExportService.validatePGNFormat,
		importPGNGames: ImportExportService.importPGNGames,
	};

	app.provide(GlobalStoreKey, globalStoreImpl);
	app.provide(ImportExportServiceKey, importExportServiceImpl);
};

/**
 * Provides mock implementations for testing/stories
 * Call this in your test setup or Histoire setup
 */
export const useMockProviders = () => {
	const mockGlobalStore: IGlobalStore = {
		importPGNGames: async (pgn: string) => {
			console.log("Mock: Would import PGN:", pgn);
			await new Promise((resolve) => setTimeout(resolve, 1000));
		},
	};

	const mockImportExportService: IImportExportService = {
		validatePGNFormat: (pgn: string) => {
			if (!pgn || pgn.trim().length === 0) {
				return { isValid: false, error: "PGN content is empty" };
			}

			// Basic PGN validation
			const lines = pgn.split("\n");
			let hasGameMoves = false;
			let hasHeaders = false;

			for (const line of lines) {
				const trimmedLine = line.trim();
				if (trimmedLine.startsWith("[") && trimmedLine.endsWith("]")) {
					hasHeaders = true;
				}
				if (trimmedLine.match(/^\d+\.?\s+[a-zA-Z]/)) {
					hasGameMoves = true;
				}
			}

			if (!hasHeaders && !hasGameMoves) {
				return { isValid: false, error: "No valid PGN headers or moves found" };
			}

			return { isValid: true };
		},
		importPGNGames: async (pgn: string) => {
			console.log("Mock service: Would import PGN:", pgn);
			await new Promise((resolve) => setTimeout(resolve, 500));
			return { success: true, data: undefined, error: undefined };
		},
	};

	provide(GlobalStoreKey, mockGlobalStore);
	provide(ImportExportServiceKey, mockImportExportService);
};
