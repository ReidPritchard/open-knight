/// Context for move categorization (extensible for future enhancements)
#[derive(Debug, Clone)]
pub struct MoveContext {
    pub is_capture: bool,
    pub is_check: bool,
    pub is_castling: bool,
    pub piece_moved: Option<String>,
    pub material_balance: Option<i32>,
}

/// Categorize a move based on evaluation difference
fn categorize_move(
    evaluation_difference: f32,
    thresholds: &MoveCategoryThresholds,
    _move_context: &MoveContext, // For future enhancement
) -> MoveCategory {
    if evaluation_difference >= thresholds.brilliant as f32 {
        MoveCategory::Brilliant
    } else if evaluation_difference >= thresholds.excellent as f32 {
        if evaluation_difference >= -5.0 {
            // Very close to best
            MoveCategory::Best
        } else {
            MoveCategory::Excellent
        }
    } else if evaluation_difference >= thresholds.good as f32 {
        MoveCategory::Good
    } else if evaluation_difference >= thresholds.inaccuracy as f32 {
        MoveCategory::Inaccuracy
    } else if evaluation_difference >= thresholds.mistake as f32 {
        MoveCategory::Mistake
    } else {
        MoveCategory::Blunder
    }
}
