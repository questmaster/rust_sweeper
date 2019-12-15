extern crate rand;

use core::cmp;

use rand::Rng;

const X_SIZE: usize = 5;
const Y_SIZE: usize = 5;

#[derive(Copy, Clone)]
struct Square {
    value: u8,
    visible: bool,
    mine: bool,
}

impl Square {
    fn new() -> Square {
        Square {
            value: 0,
            visible: true, // TODO: change default to false
            mine: false,
        }
    }
}

#[derive(Copy, Clone)]
struct Field {
    area: [[Square; Y_SIZE]; X_SIZE],
}

impl Field {
    fn new() -> Field {
        Field {
            area: [[Square::new(); Y_SIZE]; X_SIZE],
        }
    }

    fn set_mine(&mut self, x: usize, y: usize) {
        let x_lower = x as i32 - 1;
        let x_higher = x + 2;
        let y_lower = y as i32 - 1;
        let y_higher = y + 2;

        for line in cmp::max(0, x_lower) as usize..cmp::min(X_SIZE, x_higher) {
            for elem in cmp::max(0, y_lower) as usize..cmp::min(Y_SIZE, y_higher) {
                if line != x || elem != y {
                    self.area[line][elem].value += 1;
                } else {
                    self.area[line][elem].mine = true;
                }
            }
        }
    }
}

fn print_field(field: &Field) {
    println!("Field:");
    for line in 0..X_SIZE {
        for elem in 0..Y_SIZE {
            if field.area[line][elem].visible == false {
                print!(" X");
            } else if field.area[line][elem].mine == true {
                print!(" M");
            } else if field.area[line][elem].value == 0 {
                print!(" _");
            } else {
                print!(" {}", field.area[line][elem].value);
            }
        }
        println!();
    }
}

fn fill_mines_in_field(field: &mut Field, pct: u8) {
    let mut rng = rand::thread_rng();
    for line in 0..X_SIZE {
        for elem in 0..Y_SIZE {
            if rng.gen_range(0, 100) < pct {
                field.set_mine(line, elem);
            }
        }
    }
}

fn update_field(field: &mut Field) {
    // TODO
    field.set_mine(2, 2);
}

fn main() {
    let mut field = Field::new();

    // TODO add game logic

    fill_mines_in_field(&mut field, 20);

    update_field(&mut field);

    print_field(&field);
}

#[cfg(test)]
mod tests {
    use crate::{Field, Square, X_SIZE, Y_SIZE};

    #[test]
    fn square_create() {
        let sq = Square::new();

        assert_eq!(sq.mine, false);
        assert_eq!(sq.value, 0);
        assert_eq!(sq.visible, false);
    }

    #[test]
    fn field_create() {
        let f = Field::new();

        assert_eq!(f.area.len(), X_SIZE);
        for line in 0..Y_SIZE {
            assert_eq!(f.area[line].len(), Y_SIZE);
        }
    }

    #[test]
    fn field_add_mine() {
        // TODO
    }
}
