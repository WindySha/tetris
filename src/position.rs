#![allow(dead_code)]

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn left_assign(&mut self) {
        self.x -= 1;
    }

    pub fn right_assign(&mut self) {
        self.x += 1;
    }

    pub fn down_assign(&mut self) {
        self.y -= 1;
    }

    pub fn up_assign(&mut self) {
        self.y += 1;
    }

    pub fn left(&self) -> Self {
        Position::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Self {
        Position::new(self.x + 1, self.y)
    }

    pub fn down(&self) -> Self {
        Position::new(self.x, self.y - 1)
    }
}