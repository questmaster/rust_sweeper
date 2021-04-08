use crate::game_area::{GameArea, EvaluationResult};
use std::io;
use crate::ui::GameUi;

pub struct Console {

}

impl Console {
    pub fn new() -> Self {
        Console {}
    }
}

impl GameUi for Console {
    fn input_coordinate(&mut self) -> Result<(usize, usize), ()> {
        let mut x = String::new();
        let mut y = String::new();

        println!("Enter x coordinate:");
        io::stdin().read_line(&mut x).expect("Input failed.");
        println!("Enter y coordinate:");
        io::stdin().read_line(&mut y).expect("Input failed.");

        let x: usize = x.trim().parse().expect("x not a number.");
        let y: usize = y.trim().parse().expect("y not a number.");

        Ok((x, y))
    }

    fn output_game_finished(&self, evaluation: EvaluationResult, all_mines_detected: bool) -> bool {
        let mut game_finished = false;

        match evaluation {
            EvaluationResult::Mine => {
                println!("BOOMM!! You lost!");
                game_finished = true;
            }
            EvaluationResult::Nothing => {
                if all_mines_detected {
                    println!("==> You  WON !!! <==");
                    game_finished = true;
                }
            }
        }

        game_finished
    }

    fn print_area(&mut self, area: &GameArea) -> Result<(), String> {
        println!("Game area:");
        // todo not dynamic
        println!("   X  0  1  2  3  4  5  6  7  8  9");
        println!(" Y +------------------------------");
        for elem in 0..area.size_y() {
            print!(" {} |", elem);
            for line in 0..area.size_x() {
                if area.area()[line][elem].visible == false {
                    print!("  *");
                } else if area.area()[line][elem].mine == true {
                    print!("  M");
                } else if area.area()[line][elem].value == 0 {
                    print!("  _");
                } else {
                    print!("  {}", area.area()[line][elem].value);
                }
            }
            println!();
        }

        Ok(())
    }
}
