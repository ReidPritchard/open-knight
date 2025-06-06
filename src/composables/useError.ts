import {
	ErrorFactory,
	ErrorHandler,
	ErrorCategory,
	ErrorSeverity,
	type ApplicationError,
} from "../services/ErrorService";

/**
 * Composable for handling errors throughout the application
 * Provides easy access to error creation and handling
 */
export function useError() {
	/**
	 * Handle a general error
	 */
	const handleGeneralError = (
		code: "GENERAL" | "UNEXPECTED" | "NOT_IMPLEMENTED",
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError => {
		const error = ErrorFactory.general(code, message, options);
		ErrorHandler.handle(error);
		return error;
	};

	/**
	 * Handle a user interface error
	 */
	const handleUIError = (
		code: "INVALID_ACTION" | "INVALID_STATE",
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError => {
		const error = ErrorFactory.userInterface(code, message, options);
		ErrorHandler.handle(error);
		return error;
	};

	/**
	 * Handle a chess engine error
	 */
	const handleEngineError = (
		code:
			| "ENGINE_LOAD_ERROR"
			| "ENGINE_UNLOAD_ERROR"
			| "ENGINE_PROTOCOL_ERROR"
			| "ENGINE_CHILD_PROCESS_ERROR",
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError => {
		const error = ErrorFactory.chessEngine(code, message, options);
		ErrorHandler.handle(error);
		return error;
	};

	/**
	 * Handle a chess game error
	 */
	const handleGameError = (
		code: "INVALID_FEN" | "INVALID_MOVE" | "INVALID_POSITION",
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError => {
		const error = ErrorFactory.chessGame(code, message, options);
		ErrorHandler.handle(error);
		return error;
	};

	/**
	 * Handle a database error
	 */
	const handleDatabaseError = (
		code:
			| "CONNECTION_ERROR"
			| "INSERT_ERROR"
			| "QUERY_ERROR"
			| "NO_SUCH_TABLE"
			| "NO_SUCH_COLUMN",
		message: string,
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError => {
		const error = ErrorFactory.database(code, message, options);
		ErrorHandler.handle(error);
		return error;
	};

	/**
	 * Handle a native JavaScript error by converting it to ApplicationError
	 */
	const handleNativeError = (
		error: Error,
		category: ErrorCategory = ErrorCategory.GENERAL,
		code = "UNEXPECTED",
		options?: {
			recoverable?: boolean;
			severity?: ErrorSeverity;
			metadata?: Record<string, unknown>;
		},
	): ApplicationError => {
		const appError = ErrorFactory.fromNativeError(
			error,
			category,
			code,
			options,
		);
		ErrorHandler.handle(appError);
		return appError;
	};

	/**
	 * Wrap a promise to automatically handle errors
	 */
	const handleAsync = async <T>(
		promise: Promise<T>,
		fallbackValue?: T,
	): Promise<T | undefined> => {
		return ErrorHandler.handleAsync(promise, fallbackValue);
	};

	/**
	 * Handle API errors with appropriate categorization
	 */
	const handleAPIError = (
		error: unknown,
		operation: string,
		metadata?: Record<string, unknown>,
	): ApplicationError => {
		if (error instanceof Error) {
			return handleNativeError(error, ErrorCategory.DATABASE, "QUERY_ERROR", {
				severity: ErrorSeverity.HIGH,
				metadata: {
					operation,
					...metadata,
				},
			});
		}

		return handleGeneralError(
			"UNEXPECTED",
			`API operation failed: ${operation}`,
			{
				severity: ErrorSeverity.HIGH,
				metadata: {
					operation,
					originalError: String(error),
					...metadata,
				},
			},
		);
	};

	/**
	 * Handle common store operation errors
	 */
	const handleStoreError = (
		error: unknown,
		operation: string,
		metadata?: Record<string, unknown>,
	): ApplicationError => {
		if (error instanceof Error) {
			return handleNativeError(error, ErrorCategory.GENERAL, "UNEXPECTED", {
				severity: ErrorSeverity.MEDIUM,
				metadata: {
					operation,
					...metadata,
				},
			});
		}

		return handleGeneralError(
			"UNEXPECTED",
			`Store operation failed: ${operation}`,
			{
				severity: ErrorSeverity.MEDIUM,
				metadata: {
					operation,
					originalError: String(error),
					...metadata,
				},
			},
		);
	};

	return {
		// Direct access to factories and handlers
		ErrorFactory,
		ErrorHandler,
		ErrorCategory,
		ErrorSeverity,

		// Convenience methods
		handleGeneralError,
		handleUIError,
		handleEngineError,
		handleGameError,
		handleDatabaseError,
		handleNativeError,
		handleAsync,
		handleAPIError,
		handleStoreError,
	};
}
