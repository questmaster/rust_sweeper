#[derive(Copy, Clone)]
pub struct Square {
    pub value: u8,
    pub visible: bool,
    pub mine: bool,
}

impl Square {
    pub fn new() -> Self {
        Self {
            value: 0,
            visible: false,
            mine: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_create() {
        let sq = Square::new();

        assert_eq!(sq.mine, false);
        assert_eq!(sq.value, 0);
        assert_eq!(sq.visible, false);
    }
}
