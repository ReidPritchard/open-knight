import API from "../shared/api";
import type {
	ExplorerGame,
	FilterOption,
	OperationResult,
	SortOption,
} from "../shared/types";
import { ErrorCategory, withErrorHandling } from "./ErrorService";

/**
 * Import PGN games into the database
 */
export async function importPGNGames(pgn: string): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.games.import(pgn),
		ErrorCategory.DATABASE,
		"INSERT_ERROR",
		"Failed to import PGN games",
		{
			metadata: { pgnLength: pgn.length },
		},
	);
}

/**
 * Fetch games for the explorer/library view
 */
export async function fetchExplorerGames(): Promise<
	OperationResult<ExplorerGame[]>
> {
	return await withErrorHandling(
		() => API.games.list(),
		ErrorCategory.DATABASE,
		"QUERY_ERROR",
		"Failed to fetch explorer games",
		{},
	);
}

/**
 * Update a game property
 */
export async function updateGameProperty(
	gameId: number,
	property: string,
	value: string,
): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.games.update(gameId, property, value),
		ErrorCategory.DATABASE,
		"UPDATE_ERROR",
		"Failed to update game property",
		{
			metadata: { gameId, property, value },
		},
	);
}

/**
 * Reset/empty the entire database
 */
export async function resetDatabase(): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.utils.emptyDatabase(),
		ErrorCategory.DATABASE,
		"EMPTY_DATABASE_ERROR",
		"Failed to reset database",
		{},
	);
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
 * Compare two date strings
 * @param dateString1 - First date string
 * @param dateString2 - Second date string
 * @param delimiter - Delimiter used in the date string
 * @returns -1 if dateString1 is before dateString2, 0 if equal, 1 if after
 */
function compareDates(dateString1: string, dateString2: string): number {
	// Date format is "YYYY.MM.DD"
	// with "??" for unknown parts
	// ex. 2025.05.23, 2025.??.??, 2025.05.??

	// Handle null/undefined/empty dates
	if (!dateString1 && !dateString2) return 0;
	if (!dateString1) return 1; // null dates sort last
	if (!dateString2) return -1;

	// Replace "-" with "." in date strings
	dateString1 = dateString1.replace("-", ".");
	dateString2 = dateString2.replace("-", ".");

	const [year1, month1, day1] = dateString1.split(".");
	const [year2, month2, day2] = dateString2.split(".");

	// Helper function to compare individual date components
	// Returns: negative if a < b, 0 if equal, positive if a > b
	// Unknown ("??") components sort after known components
	function compareComponent(a: string, b: string): number {
		if (a === b) return 0;
		const aIsUnknown = a === "??" || a === "?" || a === "????";
		const bIsUnknown = b === "??" || b === "?" || b === "????";
		if (aIsUnknown && bIsUnknown) return 0; // both unknown
		if (aIsUnknown) return -1; // a is unknown, b is known
		if (bIsUnknown) return 1; // b is unknown, a is known
		return parseInt(a) - parseInt(b);
	}

	// Compare year, then month, then day
	let result = compareComponent(year1 || "??", year2 || "??");
	if (result !== 0) return result;

	result = compareComponent(month1 || "??", month2 || "??");
	if (result !== 0) return result;

	return compareComponent(day1 || "??", day2 || "??");
}

/**
 * Sort games by specified criteria
 */
export function sortGames(
	games: ExplorerGame[],
	sortBy: SortOption,
	sortOrder: "asc" | "desc" = "desc",
): ExplorerGame[] {
	const sortedGames = [...games].sort((a, b) => {
		let comparison = 0;

		switch (sortBy) {
			case "date":
				comparison = compareDates(
					a.date || "????.??.??",
					b.date || "????.??.??",
				);
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
	filter: FilterOption,
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
