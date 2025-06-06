import api from "../shared/api";
import type { ExplorerGame } from "../shared/types";
import { ErrorFactory, ErrorHandler } from "./ErrorService";

/**
 * Result of an import/export operation
 */
export interface ImportExportOperationResult<T = void> {
	success: boolean;
	data?: T;
	error?: string;
}

/**
 * Import PGN games into the database
 */
export async function importPGNGames(
	pgn: string,
): Promise<ImportExportOperationResult> {
	try {
		await api.games.POST.importPGNGames(pgn);
		console.log("PGN games imported successfully");
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to import PGN games";
		ErrorHandler.handle(
			ErrorFactory.database(
				"INSERT_ERROR",
				`Failed to import PGN games: ${errorMessage}`,
				{
					metadata: { pgnLength: pgn.length },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Fetch games for the explorer/library view
 */
export async function fetchExplorerGames(): Promise<
	ImportExportOperationResult<ExplorerGame[]>
> {
	try {
		const games = await api.games.GET.explorer();
		console.log("Explorer games fetched:", games.length);
		return {
			success: true,
			data: games,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to fetch games";
		ErrorHandler.handle(
			ErrorFactory.database(
				"QUERY_ERROR",
				`Failed to fetch explorer games: ${errorMessage}`,
				{},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Update a game property
 */
export async function updateGameProperty(
	gameId: number,
	property: string,
	value: string,
): Promise<ImportExportOperationResult> {
	try {
		await api.games.POST.updateProperty(gameId, property, value);
		console.log("Game property updated:", gameId, property, value);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to update game property";
		ErrorHandler.handle(
			ErrorFactory.database(
				"QUERY_ERROR",
				`Failed to update game property ${property}=${value} for game ${gameId}: ${errorMessage}`,
				{
					metadata: { gameId, property, value },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Reset/empty the entire database
 */
export async function resetDatabase(): Promise<ImportExportOperationResult> {
	try {
		await api.emptyDatabase();
		console.log("Database reset successfully");
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to reset database";
		ErrorHandler.handle(
			ErrorFactory.database(
				"CONNECTION_ERROR",
				`Failed to reset database: ${errorMessage}`,
				{},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Validate PGN format (basic validation)
 */
export function validatePGNFormat(pgn: string): {
	isValid: boolean;
	error?: string;
} {
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
}

/**
 * Sort games by specified criteria
 */
export function sortGames(
	games: ExplorerGame[],
	sortBy: "date" | "event" | "white" | "black" | "result" | "opening",
	sortOrder: "asc" | "desc" = "desc",
): ExplorerGame[] {
	const sortedGames = [...games].sort((a, b) => {
		let comparison = 0;

		switch (sortBy) {
			case "date":
				comparison = (a.date || "").localeCompare(b.date || "");
				break;
			case "event":
				comparison = (a.tournament?.name || "").localeCompare(
					b.tournament?.name || "",
				);
				break;
			case "white":
				comparison = (a.white_player?.name || "").localeCompare(
					b.white_player?.name || "",
				);
				break;
			case "black":
				comparison = (a.black_player?.name || "").localeCompare(
					b.black_player?.name || "",
				);
				break;
			case "result":
				comparison = (a.result || "").localeCompare(b.result || "");
				break;
			case "opening":
				comparison = (a.opening?.name || "").localeCompare(
					b.opening?.name || "",
				);
				break;
			default:
				comparison = 0;
		}

		return sortOrder === "desc" ? -comparison : comparison;
	});

	return sortedGames;
}

/**
 * Filter games by criteria
 */
export function filterGames(
	games: ExplorerGame[],
	filter: "all" | "favorites" | "tags",
	filterTags: string[] = [],
): ExplorerGame[] {
	switch (filter) {
		case "all":
			return games;
		case "favorites":
			// TODO: Implement favorites system
			return games;
		case "tags":
			if (filterTags.length === 0) return games;
			return games.filter((game) => {
				const gameTags = game.tags || {};
				return filterTags.some(
					(tag) =>
						Object.keys(gameTags).includes(tag) ||
						Object.values(gameTags).includes(tag),
				);
			});
		default:
			return games;
	}
}

/**
 * Search games by text
 */
export function searchGames(
	games: ExplorerGame[],
	searchText: string,
): ExplorerGame[] {
	if (!searchText || searchText.trim().length === 0) {
		return games;
	}

	const searchLower = searchText.toLowerCase();

	return games.filter((game) => {
		const searchFields = [
			game.white_player?.name,
			game.black_player?.name,
			game.tournament?.name,
			game.opening?.name,
			game.result,
			game.date,
		];

		return searchFields.some((field) =>
			field?.toLowerCase().includes(searchLower),
		);
	});
}

/**
 * Get game statistics
 */
export function getGameStatistics(games: ExplorerGame[]): {
	totalGames: number;
	whiteWins: number;
	blackWins: number;
	draws: number;
	mostPlayedOpening?: string;
	dateRange?: { earliest: string; latest: string };
} {
	const stats = {
		totalGames: games.length,
		whiteWins: 0,
		blackWins: 0,
		draws: 0,
		mostPlayedOpening: undefined as string | undefined,
		dateRange: undefined as { earliest: string; latest: string } | undefined,
	};

	if (games.length === 0) return stats;

	const openingCounts: Record<string, number> = {};
	const dates: string[] = [];

	for (const game of games) {
		// Count results
		switch (game.result) {
			case "1-0":
				stats.whiteWins++;
				break;
			case "0-1":
				stats.blackWins++;
				break;
			case "1/2-1/2":
				stats.draws++;
				break;
		}

		// Count openings
		if (game.opening?.name) {
			openingCounts[game.opening.name] =
				(openingCounts[game.opening.name] || 0) + 1;
		}

		// Collect dates
		if (game.date) {
			dates.push(game.date);
		}
	}

	// Find most played opening
	if (Object.keys(openingCounts).length > 0) {
		stats.mostPlayedOpening = Object.entries(openingCounts).reduce((a, b) =>
			openingCounts[a[0]] > openingCounts[b[0]] ? a : b,
		)[0];
	}

	// Calculate date range
	if (dates.length > 0) {
		dates.sort();
		stats.dateRange = {
			earliest: dates[0],
			latest: dates[dates.length - 1],
		};
	}

	return stats;
}
