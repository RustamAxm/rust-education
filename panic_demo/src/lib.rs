#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value should be between 1 and 100 {value}!!!");
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}
