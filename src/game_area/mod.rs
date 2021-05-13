use core::cmp;
use percent::Percent;
use rand::Rng;
use square::Square;

pub mod percent;
mod square;

const X_SIZE: usize = 10;
const Y_SIZE: usize = 10;

#[derive(Debug, PartialEq)]
pub enum EvaluationResult {
    Mine,
    Nothing,
}

#[derive(Copy, Clone)]
pub struct GameArea {
    area: [[Square; Y_SIZE]; X_SIZE],
    size_x: usize,
    size_y: usize,
}

impl GameArea {
    pub fn new(perc: Percent) -> GameArea {
        let mut f = GameArea {
            area: [[Square::new(); Y_SIZE]; X_SIZE],
            size_x: X_SIZE,
            size_y: Y_SIZE,
        };

        f.fill_mines_in_area(perc);

        f
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

        if cfg!(debug_assertions) {
            println!("[Debug] Placing mine at ({}, {}). Psst!", x, y);
        }

        // todo: maybe add iterators
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

    fn fill_mines_in_area(&mut self, pct: Percent) {
        let mut rng = rand::thread_rng();

        let mine_cnt = ((self.size_x() * self.size_y()) as f32 * pct.value()) as usize;

        for _i in 0..mine_cnt {
            let x = rng.gen_range(0..self.size_x());
            let y = rng.gen_range(0..self.size_y());

            self.set_mine(x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_area_create() {
        let f = GameArea::new(Percent::new(0));

        assert_eq!(f.area.len(), X_SIZE);
        for line in 0..Y_SIZE {
            assert_eq!(f.area[line].len(), Y_SIZE);
        }
    }

    #[test]
    fn game_area_add_mine() {
        let mut f = GameArea::new(Percent::new(0));

        f.set_mine(3, 3);

        assert_eq!(f.evaluate_square(3, 3), EvaluationResult::Mine);
        assert_eq!(f.evaluate_square(3, 4), EvaluationResult::Nothing);
    }
}
