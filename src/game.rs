use std::time::Duration;

use crate::render::Dimension;

pub const WIDTH: Dimension = 100;
pub const HEIGHT: Dimension = 20;
pub const UPDATE_INTERVAL: Duration = Duration::from_millis(250);

pub type Coord = i64;

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: Coord,
    pub y: Coord,
}

impl Pos {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
