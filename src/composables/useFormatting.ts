import type { ChessEvaluation } from "../shared/bindings";

export function useFormatting() {
	const formatEvaluation = (evaluation: ChessEvaluation): string => {
		if (!evaluation || evaluation.score == null) return "?";
		const score = evaluation.score / 100;
		if (evaluation.eval_type === "mate") {
			return `#${evaluation.score > 0 ? "+" : ""}${evaluation.score}`;
		}
		return `${score > 0 ? "+" : ""}${score.toFixed(2)}`;
	};

	const formatTime = (ms: number | null | undefined): string => {
		if (!ms) return "0:00";
		const seconds = Math.floor(ms / 1000);
		const minutes = Math.floor(seconds / 60);
		const remainingSeconds = seconds % 60;
		return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
	};

	return {
		formatEvaluation,
		formatTime,
	};
}
