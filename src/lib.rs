use std::time::Duration;

use game_area::percent::Percent;
use game_area::GameArea;
use game_area_ui::GameUi;

use crate::game_area::EvaluationResult;

mod game_area;
mod game_area_ui;

pub enum GameUiType {
    Console,
    SDL,
}

pub struct SweeperGame {
    area: GameArea,
    game_ui: Box<dyn GameUi>,
}

impl SweeperGame {
    pub fn new(ui_type: GameUiType) -> Self {
        println!("Preparing game area...");
        let area = GameArea::new(Percent::new(10));

        println!("Let's start!");
        let game_ui = create_ui(ui_type);

        Self { area, game_ui }
    }

    pub fn start(&mut self) {
        let mut x: usize;
        let mut y: usize;
        'running: loop {
            // Handle events
            match self.game_ui.input_coordinate() {
                Ok((xx, yy)) => {
                    x = xx;
                    y = yy;
                }
                Err(true) => {
                    break 'running;
                }
                _ => {
                    self.game_ui
                        .print_area(&self.area, &EvaluationResult::Nothing)
                        .unwrap();
                    continue 'running;
                }
            }

            // Update
            let evaluation = self.area.evaluate_square(x, y);

            // Render
            self.game_ui.print_area(&self.area, &evaluation).unwrap();

            // Time management!
            if evaluation != EvaluationResult::Nothing {
                ::std::thread::sleep(Duration::new(5, 0));
                break 'running;
            } else {
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        }
    }
}

fn create_ui(ui_type: GameUiType) -> Box<dyn GameUi> {
    let game_ui: Box<dyn GameUi>;
    match ui_type {
        GameUiType::Console => {
            game_ui = Box::new(game_area_ui::console_ui::Console::new());
        }
        GameUiType::SDL => {
            game_ui = Box::new(game_area_ui::sdl_ui::Sdl::new());
        }
    }
    game_ui
}
