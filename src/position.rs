#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    pub r: usize,
    pub c: usize,
}

impl Position {
    pub fn apply_move(self, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up => match self.r.checked_sub(1) {
                Some(r) => Some(Self { r: r, c: self.c }),
                None => None,
            },
            Direction::Down => match self.r.checked_add(1) {
                Some(r) => Some(Self { r: r, c: self.c }),
                None => None,
            },
            Direction::Left => match self.c.checked_sub(1) {
                Some(c) => Some(Self { r: self.r, c: c }),
                None => None,
            },
            Direction::Right => match self.c.checked_add(1) {
                Some(c) => Some(Self { r: self.r, c: c }),
                None => None,
            },
        }
    }
}
