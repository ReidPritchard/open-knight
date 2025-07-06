import { warn } from "@tauri-apps/plugin-log";
import type { OperationResult } from "../shared/types";

/**
 * Categories of errors that can be thrown by the application.
 */
export enum ErrorCategory {
	/**
	 * General errors that do not fit into other categories.
	 */
	GENERAL = "GENERAL",
	/**
	 * Errors related to the user interface.
	 */
	USER_INTERFACE = "USER_INTERFACE",
	/**
	 * Errors related to chess engines.
	 * Loading, unloading, protocol errors, child process errors, etc.
	 */
	CHESS_ENGINE = "CHESS_ENGINE",
	/**
	 * Errors related to chess games.
	 * Invalid FEN, invalid move, etc.
	 */
	CHESS_GAME = "CHESS_GAME",
	/**
	 * Errors related to the database.
	 * Database connection errors, db insert errors, db query errors, etc.
	 */
	DATABASE = "DATABASE",
}

/**
 * Error codes grouped by category.
 */
export const ErrorCodes = {
	[ErrorCategory.GENERAL]: {
		GENERAL: "GENERAL",
		UNEXPECTED: "UNEXPECTED", // Unexpected argument, state, etc.
		NOT_IMPLEMENTED: "NOT_IMPLEMENTED", // Feature not implemented.
	},
	[ErrorCategory.USER_INTERFACE]: {
		INVALID_ACTION: "INVALID_ACTION", // Invalid action, e.g. clicking on a disabled button.
		INVALID_STATE: "INVALID_STATE", // Invalid state, e.g. trying to open a panel that is already open.
	},
	[ErrorCategory.CHESS_ENGINE]: {
		ENGINE_LOAD_ERROR: "ENGINE_LOAD_ERROR", // Engine failed to load.
		ENGINE_UNLOAD_ERROR: "ENGINE_UNLOAD_ERROR", // Engine failed to unload.
		ENGINE_PROTOCOL_ERROR: "ENGINE_PROTOCOL_ERROR", // Engine protocol error, e.g. engine not responding to protocol commands.
		ENGINE_CHILD_PROCESS_ERROR: "ENGINE_CHILD_PROCESS_ERROR", // Engine child process error, e.g. engine process crashed.
	},
	[ErrorCategory.CHESS_GAME]: {
		INVALID_FEN: "INVALID_FEN", // Invalid FEN, e.g. invalid piece, invalid square, etc.
		INVALID_MOVE: "INVALID_MOVE", // Invalid move, moving while in check, etc.
		INVALID_POSITION: "INVALID_POSITION", // Invalid position, e.g. invalid piece, invalid square, etc.
	},
	[ErrorCategory.DATABASE]: {
		CONNECTION_ERROR: "CONNECTION_ERROR", // Database connection error, e.g. database not found.
		INSERT_ERROR: "INSERT_ERROR", // Database insert error, e.g. missing required fields.
		QUERY_ERROR: "QUERY_ERROR", // Database query error, e.g. wrong fields for table.
		NO_SUCH_TABLE: "NO_SUCH_TABLE", // Database table does not exist.
		NO_SUCH_COLUMN: "NO_SUCH_COLUMN", // Database column does not exist.
	},
} as const;

/**
 * Type for error codes within each category
 */
type ErrorCodesType = typeof ErrorCodes;
type GeneralErrorCode = keyof ErrorCodesType[ErrorCategory.GENERAL];
type UIErrorCode = keyof ErrorCodesType[ErrorCategory.USER_INTERFACE];
type EngineErrorCode = keyof ErrorCodesType[ErrorCategory.CHESS_ENGINE];
type GameErrorCode = keyof ErrorCodesType[ErrorCategory.CHESS_GAME];
type DatabaseErrorCode = keyof ErrorCodesType[ErrorCategory.DATABASE];

/**
 * Error severity levels
 */
export enum ErrorSeverity {
	LOW = "LOW",
	MEDIUM = "MEDIUM",
	HIGH = "HIGH",
	CRITICAL = "CRITICAL",
}

/**
 * Base class for all application errors.
 *
 * @param message - The error message.
 * @param category - The category of the error.
 * @param code - The error code.
 * @param recoverable - Whether the error is recoverable.
 * @param severity - The severity level of the error.
 * @param metadata - Additional error context or metadata.
 */
export class ApplicationError extends Error {
	constructor(
		message: string,
		public category: string,
		public code: string,
		public recoverable = true,
		public severity: ErrorSeverity = ErrorSeverity.MEDIUM,
		public metadata?: Record<string, unknown>,
	) {
		super(message);
		this.name = "ApplicationError";
	}

	/**
	 * Convert error to a JSON-serializable object
	 */
	toJSON() {
		return {
			name: this.name,
			message: this.message,
			category: this.category,
			code: this.code,
			recoverable: this.recoverable,
			severity: this.severity,
			metadata: this.metadata,
			stack: this.stack,
		};
	}

	/**
	 * Check if error is critical (non-recoverable)
	 */
	isCritical(): boolean {
		return this.severity === ErrorSeverity.CRITICAL || !this.recoverable;
	}
}

/**
 * Error factory for creating typed errors
 */
export const ErrorFactory = {
	/**
	 * Create a general error
	 */
	general(
		code: GeneralErrorCode,
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		return new ApplicationError(
			message,
			ErrorCategory.GENERAL,
			ErrorCodes[ErrorCategory.GENERAL][code],
			options?.recoverable ?? true,
			options?.severity ?? ErrorSeverity.MEDIUM,
			options?.metadata,
		);
	},

	/**
	 * Create a user interface error
	 */
	userInterface(
		code: UIErrorCode,
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		return new ApplicationError(
			message,
			ErrorCategory.USER_INTERFACE,
			ErrorCodes[ErrorCategory.USER_INTERFACE][code],
			options?.recoverable ?? true,
			options?.severity ?? ErrorSeverity.LOW,
			options?.metadata,
		);
	},

	/**
	 * Create a chess engine error
	 */
	chessEngine(
		code: EngineErrorCode,
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		return new ApplicationError(
			message,
			ErrorCategory.CHESS_ENGINE,
			ErrorCodes[ErrorCategory.CHESS_ENGINE][code],
			options?.recoverable ?? true,
			options?.severity ?? ErrorSeverity.HIGH,
			options?.metadata,
		);
	},

	/**
	 * Create a chess game error
	 */
	chessGame(
		code: GameErrorCode,
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		return new ApplicationError(
			message,
			ErrorCategory.CHESS_GAME,
			ErrorCodes[ErrorCategory.CHESS_GAME][code],
			options?.recoverable ?? true,
			options?.severity ?? ErrorSeverity.MEDIUM,
			options?.metadata,
		);
	},

	/**
	 * Create a database error
	 */
	database(
		code: DatabaseErrorCode,
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		return new ApplicationError(
			message,
			ErrorCategory.DATABASE,
			ErrorCodes[ErrorCategory.DATABASE][code],
			options?.recoverable ?? false,
			options?.severity ?? ErrorSeverity.HIGH,
			options?.metadata,
		);
	},

	/**
	 * Create an error from a native JavaScript Error
	 */
	fromNativeError(
		error: Error,
		category: ErrorCategory = ErrorCategory.GENERAL,
		code = "UNEXPECTED",
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		const appError = new ApplicationError(
			error.message,
			category,
			code,
			options?.recoverable ?? true,
			options?.severity ?? ErrorSeverity.MEDIUM,
			{
				originalError: error.name,
				originalStack: error.stack,
				...options?.metadata,
			},
		);

		// Preserve original stack trace
		if (error.stack) {
			appError.stack = error.stack;
		}

		return appError;
	},
} as const;

/**
 * Mapping object for ErrorFactory methods by category
 */
type ErrorFactoryFunction = (
	code: string,
	message: string,
	options?: { metadata?: Record<string, unknown> },
) => ApplicationError;

const errorFactoryMap: Record<ErrorCategory, ErrorFactoryFunction> = {
	[ErrorCategory.GENERAL]: (code, message, options) =>
		ErrorFactory.general(code as GeneralErrorCode, message, options),
	[ErrorCategory.USER_INTERFACE]: (code, message, options) =>
		ErrorFactory.userInterface(code as UIErrorCode, message, options),
	[ErrorCategory.CHESS_ENGINE]: (code, message, options) =>
		ErrorFactory.chessEngine(code as EngineErrorCode, message, options),
	[ErrorCategory.CHESS_GAME]: (code, message, options) =>
		ErrorFactory.chessGame(code as GameErrorCode, message, options),
	[ErrorCategory.DATABASE]: (code, message, options) =>
		ErrorFactory.database(code as DatabaseErrorCode, message, options),
};

// Private state for error handler
const errorListeners: Array<(error: ApplicationError) => void> = [];

/**
 * Error handler utilities
 */
export const ErrorHandler = {
	/**
	 * Register an error listener
	 */
	addListener(listener: (error: ApplicationError) => void): void {
		errorListeners.push(listener);
	},

	/**
	 * Remove an error listener
	 */
	removeListener(listener: (error: ApplicationError) => void): void {
		const index = errorListeners.indexOf(listener);
		if (index > -1) {
			errorListeners.splice(index, 1);
		}
	},

	/**
	 * Handle an error by notifying all listeners
	 */
	handle(error: ApplicationError): void {
		// Log error for debugging
		warn(
			`Error occurred: ${error.message} (Category: ${error.category}, Code: ${error.code}, Severity: ${error.severity})`,
		);

		// Notify all listeners
		for (const listener of errorListeners) {
			try {
				listener(error);
			} catch (listenerError) {
				// If a listener throws an error, log it but continue notifying others
				warn(
					`Error in error listener: ${listenerError instanceof Error ? listenerError.message : String(listenerError)}`,
				);
			}
		}
	},

	/**
	 * Create and handle an error in one call
	 */
	createAndHandle(
		category: ErrorCategory,
		code: string,
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError {
		const error = new ApplicationError(
			message,
			category,
			code,
			options?.recoverable ?? true,
			options?.severity ?? ErrorSeverity.MEDIUM,
			options?.metadata,
		);

		ErrorHandler.handle(error);
		return error;
	},

	/**
	 * Wrap a promise to automatically handle ApplicationErrors
	 */
	async handleAsync<T>(
		promise: Promise<T>,
		fallbackValue?: T,
	): Promise<T | undefined> {
		try {
			return await promise;
		} catch (error) {
			if (error instanceof ApplicationError) {
				ErrorHandler.handle(error);
				return fallbackValue;
			}

			// Convert native errors to ApplicationErrors
			const appError = ErrorFactory.fromNativeError(
				error instanceof Error ? error : new Error(String(error)),
			);
			ErrorHandler.handle(appError);
			return fallbackValue;
		}
	},

	/**
	 * Check if an error is recoverable
	 */
	isRecoverable(error: unknown): boolean {
		if (error instanceof ApplicationError) {
			return error.recoverable;
		}
		return true; // Assume recoverable for non-ApplicationErrors
	},

	/**
	 * Get error severity
	 */
	getSeverity(error: unknown): ErrorSeverity {
		if (error instanceof ApplicationError) {
			return error.severity;
		}
		return ErrorSeverity.MEDIUM; // Default severity for non-ApplicationErrors
	},
} as const;

/**
 * Type guards for error checking
 */
export const ErrorChecks = {
	isApplicationError: (error: unknown): error is ApplicationError => {
		return error instanceof ApplicationError;
	},

	isGeneralError: (error: unknown): error is ApplicationError => {
		return (
			error instanceof ApplicationError &&
			error.category === ErrorCategory.GENERAL
		);
	},

	isUIError: (error: unknown): error is ApplicationError => {
		return (
			error instanceof ApplicationError &&
			error.category === ErrorCategory.USER_INTERFACE
		);
	},

	isEngineError: (error: unknown): error is ApplicationError => {
		return (
			error instanceof ApplicationError &&
			error.category === ErrorCategory.CHESS_ENGINE
		);
	},

	isGameError: (error: unknown): error is ApplicationError => {
		return (
			error instanceof ApplicationError &&
			error.category === ErrorCategory.CHESS_GAME
		);
	},

	isDatabaseError: (error: unknown): error is ApplicationError => {
		return (
			error instanceof ApplicationError &&
			error.category === ErrorCategory.DATABASE
		);
	},

	isCriticalError: (error: unknown): boolean => {
		if (error instanceof ApplicationError) {
			return error.isCritical();
		}
		return false;
	},
} as const;

/**
 * Wrap an operation in error handling
 *
 * @param operation - The operation to wrap.
 * @param errorCategory - The category of the error.
 * @param errorCode - The error code.
 * @param fallbackMessage - The fallback message to use if the operation fails.
 * @param metadata - Additional error context or metadata.
 */
export async function withErrorHandling<T>(
	operation: () => Promise<T>,
	errorCategory: ErrorCategory,
	errorCode: string,
	fallbackMessage: string,
	metadata?: Record<string, unknown>,
): Promise<OperationResult<T>> {
	try {
		const result = await operation();
		return { success: true, data: result };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : fallbackMessage;

		const factoryFunction = errorFactoryMap[errorCategory];
		const applicationError = factoryFunction
			? factoryFunction(errorCode, errorMessage, { metadata })
			: ErrorFactory.general("UNEXPECTED", errorMessage, { metadata });

		ErrorHandler.handle(applicationError);
		return { success: false, error: errorMessage };
	}
}
