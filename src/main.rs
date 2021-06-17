use rust_sweeper::{GameUiType, SweeperGame};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rust_Sweeper",
    about = "A mine sweeper clone written in rust programming language."
)]
struct Opt {
    /// Activate SDL user interface
    #[structopt(
        short = "s",
        long = "sdl_ui",
        help = "Activate SDL GUI replacing the Console version."
    )]
    use_sdl_ui: bool,
}

fn main() {
    let opts = Opt::from_args();

    let ui_type;
    if opts.use_sdl_ui {
        ui_type = GameUiType::SDL;
    } else {
        ui_type = GameUiType::Console;
    }

    let mut game = SweeperGame::new(ui_type);
    game.start();
}
