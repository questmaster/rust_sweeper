use std::time::Duration;

use game_area::GameArea;
use game_area::percent::Percent;
use game_area_ui::GameUi;

mod game_area;
mod game_area_ui;

pub enum GameUiType {
    Console,
    SDL,
}

pub struct SweeperGame {
    area: GameArea,
    game_ui: Box<dyn GameUi>,
    game_finished: bool,
}

impl SweeperGame {
    pub fn new(ui_type: GameUiType) -> Self {
        println!("Preparing game area...");
        let area = GameArea::new(Percent::new(10));

        println!("Let's start!");
        let game_ui = create_ui(ui_type);

        let game_finished = false;

        SweeperGame {
            area,
            game_ui,
            game_finished,
        }
    }

    pub fn start(&mut self) {
        let mut x = 0usize;
        let mut y = 0usize;
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
                    self.game_ui.print_area(&self.area).unwrap(); // Terminal
                    continue 'running;
                }
            }

            // Update
            let evaluation = self.area.evaluate_square(x, y);
            self.game_finished = self
                .game_ui
                .output_game_finished(evaluation, self.area.all_mines_detected()); // Terminal

            // Render
            self.game_ui.print_area(&self.area).unwrap(); // Terminal

            // Time management!
            if self.game_finished {
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
