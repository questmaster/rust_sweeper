use std::time::Duration;

use game_area::GameArea;
use game_area::percent::Percent;
use crate::ui::GameUi;
use std::option::Option::None;
use std::env;

mod game_area;
mod game_area_ui;


mod ui {
    use crate::game_area::{EvaluationResult, GameArea};

    /// Trait to define a common interface for different UIs.
    pub trait GameUi {
        /// Get new input coordinate from UI.
        fn input_coordinate(&mut self) -> Result<(usize, usize), ()>;

        /// Game result reporting in UI.
        fn output_game_finished(&self, evaluation: EvaluationResult, all_mines_detected: bool) -> bool;

        /// Output game state via UI.
        fn print_area(&mut self, area: &GameArea) -> Result<(), String>;
    }
}


fn process_cmd_line_params() -> Box<dyn GameUi> {
    let args: Vec<String> = env::args().collect();
    let game_ui: Box<dyn GameUi>;
    match &args.get(1) {
        Some(text) => {
            if **text == "console".to_string()
            {
                game_ui = Box::new(game_area_ui::console_ui::Console::new());
            } else {
                game_ui = Box::new(game_area_ui::sdl_ui::Sdl::new());
            }
        },
        None => {
            game_ui = Box::new(game_area_ui::console_ui::Console::new());
        }
    }
    game_ui
}

fn main() {
    println!("Preparing game area...");
    let mut area = GameArea::new(Percent::new(10));

    println!("Let's start!");
    let mut game_ui = process_cmd_line_params();

    let mut game_finished = false;
    let mut x = 0usize;
    let mut y = 0usize;
    'running: loop {
        // Handle events
        match game_ui.input_coordinate()
        {
            Ok((xx, yy)) => { x = xx; y= yy; }
            Err(_) => { break 'running; }
        }

        // Update
        let evaluation = area.evaluate_square(x, y);
        game_finished = game_ui.output_game_finished(evaluation, area.all_mines_detected()); // Terminal

        // Render
        game_ui.print_area(&area).unwrap(); // Terminal

        // Time management!
        if game_finished {
            break 'running;
        } else {
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
