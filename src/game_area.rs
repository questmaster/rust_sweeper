use core::cmp;

const X_SIZE: usize = 10;
const Y_SIZE: usize = 10;

pub enum EvaluationResult {
    Mine,
    Nothing,
}

#[derive(Copy, Clone)]
pub struct Square {
    value: u8,
    visible: bool,
    mine: bool,
}

impl Square {
    pub fn new() -> Square {
        Square {
            value: 0,
            visible: false,
            mine: false,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn mine(&self) -> bool {
        self.mine
    }
}

#[derive(Copy, Clone)]
pub struct GameArea {
    area: [[Square; Y_SIZE]; X_SIZE],
    size_x: usize,
    size_y: usize,
}

impl GameArea {
    pub fn new() -> GameArea {
        GameArea {
            area: [[Square::new(); Y_SIZE]; X_SIZE],
            size_x: X_SIZE,
            size_y: Y_SIZE,
        }
    }

    pub fn area(&self) -> &[[Square; Y_SIZE]; X_SIZE] {
        &self.area
    }

    pub fn size_x(&self) -> usize {
        self.size_x
    }

    pub fn size_y(&self) -> usize {
        self.size_y
    }

    pub fn set_mine(&mut self, x: usize, y: usize) {
        let x_lower = x as i32 - 1;
        let x_higher = x + 2;
        let y_lower = y as i32 - 1;
        let y_higher = y + 2;

        if self.area[x][y].mine == true {
            return;
        }

        // todo debug output
        println!("Placing mine at ({}, {}). Psst!", x, y);

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

    pub fn evaluate_square(&mut self, x: usize, y: usize) -> EvaluationResult {
        let mut result = EvaluationResult::Mine;

        let x_lower = x as i32 - 1;
        let x_higher = x + 2;
        let y_lower = y as i32 - 1;
        let y_higher = y + 2;
        // TODO extract surrounding fkt from this and above fkt.

        self.area[x][y].visible = true;

        if self.area[x][y].mine == false {
            if self.area[x][y].value == 0 {
                for line in cmp::max(0, x_lower) as usize..cmp::min(X_SIZE, x_higher) {
                    for elem in cmp::max(0, y_lower) as usize..cmp::min(Y_SIZE, y_higher) {
                        if !self.area[line][elem].visible {
                            self.evaluate_square(line, elem);
                        }
                    }
                }
            }

            result = EvaluationResult::Nothing;
        }

        result
    }

    pub fn all_mines_detected(&self) -> bool {
        let mut result = true;

        for line in 0..X_SIZE {
            for elem in 0..Y_SIZE {
                if !self.area[line][elem].mine && !self.area[line][elem].visible {
                    result = false;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::game_area::{GameArea, Square, X_SIZE, Y_SIZE};

    #[test]
    fn square_create() {
        let sq = Square::new();

        assert_eq!(sq.mine, false);
        assert_eq!(sq.value, 0);
        assert_eq!(sq.visible, false);
    }

    #[test]
    fn field_create() {
        let f = GameArea::new();

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
