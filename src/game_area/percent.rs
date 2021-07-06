pub struct Percent {
    value: f32,
}

impl Percent {
    pub fn new(value: usize) -> Self {
        if value > 100 {
            panic!("Percentage out of range!");
        }

        Self {
            value: (value as f32 / 100.0),
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_create() {
        let p = Percent::new(10);

        assert_eq!(p.value(), 0.1f32);
    }

    #[test]
    #[should_panic]
    fn percent_too_large() {
        let _p = Percent::new(110);
    }
}
