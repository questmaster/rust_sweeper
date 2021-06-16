use rust_sweeper::{GameUiType, SweeperGame};
use std::env;

fn process_cmd_line_params() -> GameUiType {
    let args: Vec<String> = env::args().collect();

    match &args.get(1) {
        Some(text) => {
            if **text == "console".to_string() {
                GameUiType::Console
            } else {
                GameUiType::SDL
            }
        }
        None => GameUiType::Console,
    }
}

fn main() {
    let ui_type = process_cmd_line_params();
    let mut game = SweeperGame::new(ui_type);
    game.start();
}
