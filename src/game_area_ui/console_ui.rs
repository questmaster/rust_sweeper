use std::io;

use crate::game_area::{EvaluationResult, GameArea};

use super::GameUi;

pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Console {}
    }
}

impl GameUi for Console {
    fn input_coordinate(&mut self) -> Result<(usize, usize), bool> {
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

    fn print_area(&mut self, area: &GameArea, evaluation: &EvaluationResult) -> Result<(), String> {
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

        match evaluation {
            EvaluationResult::Mine => {
                println!("BOOMM!! You lost!");
            }
            EvaluationResult::Won => {
                println!("==> You  WON !!! <==");
            }
            _ => {
                // noting to do
            }
        }

        Ok(())
    }
}
