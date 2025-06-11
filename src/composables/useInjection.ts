import type { InjectionKey } from "vue";
import type { OperationResult } from "../shared/types";

// Define interfaces for the services we want to inject
export interface IGlobalStore {
	importPGNGames(pgn: string): Promise<void>;
	// Add other global store methods as needed
}

export interface IImportExportService {
	validatePGNFormat(pgn: string): { isValid: boolean; error?: string };
	importPGNGames(pgn: string): Promise<OperationResult>;
	// Add other service methods as needed
}

// Define injection keys with proper typing
export const GlobalStoreKey: InjectionKey<IGlobalStore> = Symbol("GlobalStore");
export const ImportExportServiceKey: InjectionKey<IImportExportService> =
	Symbol("ImportExportService");

// Helper function to create a mock implementation
export const createMockGlobalStore = (): IGlobalStore => ({
	importPGNGames: async (pgn: string) => {
		console.log("Mock: Would import PGN:", pgn);
		await new Promise((resolve) => setTimeout(resolve, 1000));
	},
});

export const createMockImportExportService = (): IImportExportService => ({
	validatePGNFormat: (pgn: string) => {
		if (!pgn || pgn.trim().length === 0) {
			return { isValid: false, error: "PGN content is empty" };
		}

		// Basic checks for PGN format
		const lines = pgn.split("\n");
		let hasGameMoves = false;
		let hasHeaders = false;

		for (const line of lines) {
			const trimmedLine = line.trim();

			// Check for PGN headers
			if (trimmedLine.startsWith("[") && trimmedLine.endsWith("]")) {
				hasHeaders = true;
			}

			// Check for move notation (simplified check)
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
});
