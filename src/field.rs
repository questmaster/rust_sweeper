use core::cmp;

const X_SIZE: usize = 5;
const Y_SIZE: usize = 5;

pub enum SearchResult {
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
            visible: true,
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
pub struct Field {
    area: [[Square; Y_SIZE]; X_SIZE],
    size_x: usize,
    size_y: usize,
}

impl Field {
    pub fn new() -> Field {
        Field {
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

    pub fn search_square(&self, x: usize, y: usize) -> SearchResult {
        // TODO implement function
        return SearchResult::Mine;
    }
}

#[cfg(test)]
mod tests {
    use crate::field::{Field, Square, X_SIZE, Y_SIZE};

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
