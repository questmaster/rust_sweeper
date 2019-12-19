extern crate rand;

mod field;
mod field_ui;

use crate::field::SearchResult;
use field::Field;
use rand::Rng;
use std::io;

struct Percent {
    value: f32,
}

impl Percent {
    fn new(value: usize) -> Percent {
        if value > 100 {
            panic!("Percentage out of range!");
        }

        Percent {
            value: (value as f32 / 100.0),
        }
    }

    fn value(&self) -> f32 {
        self.value
    }
}

fn fill_mines_in_field(field: &mut Field, pct: Percent) {
    let mut rng = rand::thread_rng();

    let mine_cnt = ((field.size_x() * field.size_y()) as f32 * pct.value()) as usize;

    for _i in 0..mine_cnt {
        let x = rng.gen_range(0, field.size_x());
        let y = rng.gen_range(0, field.size_y());

        field.set_mine(x, y);
    }
}

fn main() {
    println!("Prepairing field...");
    let mut field = Field::new();
    fill_mines_in_field(&mut field, Percent::new(20));

    println!("Let's start!");
    loop {
        let mut x = String::new();
        let mut y = String::new();

        println!("Enter x coordinate:");
        io::stdin().read_line(&mut x).expect("Input failed.");
        println!("Enter y coordinate:");
        io::stdin().read_line(&mut y).expect("Input failed.");

        let x: usize = x.trim().parse().expect("x not a number.");
        let y: usize = y.trim().parse().expect("y not a number.");

        let search = field.search_square(x, y);

        field_ui::print_field(&field);

        match search {
            SearchResult::Mine => {
                println!("BOOMM!! You lost!");
                break;
            }
            SearchResult::Nothing => {}
        }

        // TODO Implement game finish.
    }
}
