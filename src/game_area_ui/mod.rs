use crate::game_area::{EvaluationResult, GameArea};

pub mod console_ui;
pub mod sdl_ui;

/// Trait to define a common interface for different UIs.
pub trait GameUi {
    /// Get new input coordinate from UI.
    fn input_coordinate(&mut self) -> Result<(usize, usize), bool>;

    /// Output game state via UI.
    fn print_area(&mut self, area: &GameArea, evaluation: &EvaluationResult) -> Result<(), String>;
}
