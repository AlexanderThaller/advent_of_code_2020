#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Coordinate {
    pub fn step_right(self, steps: usize) -> Self {
        Self {
            x: self.x + steps,
            y: self.y,
        }
    }

    pub fn step_down(self, steps: usize) -> Self {
        Self {
            x: self.x,
            y: self.y + steps,
        }
    }
}
