#[derive(Clone, Copy, Debug)]
pub struct Year {
    index: u16,
}

impl Year {
    pub fn new(index: u16) -> Year {
        Year { index }
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn is_leap(&self) -> bool {
        self.index % 4 == 0 && (self.index % 100 != 0 || self.index % 400 == 0)
    }
}
