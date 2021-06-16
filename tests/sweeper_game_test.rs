use rust_sweeper::{GameUiType, SweeperGame};

#[test]
fn create_game_structure() {
    let _g = SweeperGame::new(GameUiType::Console);

    // creation without panic for now
}
