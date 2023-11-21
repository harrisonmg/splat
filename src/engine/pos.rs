use std::fmt::Display;

pub type Coord = f32;

pub trait Signed: Copy {
    fn sign(self) -> Self;
}

impl Signed for Coord {
    fn sign(self) -> Self {
        if self > 0.0 {
            1.0
        } else if self < 0.0 {
            -1.0
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: Coord,
    pub y: Coord,
}

impl Pos {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn scale(&self, scalar: Coord) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn magnitude(&self) -> Coord {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {:.3}, y: {:.3} }}", self.x, self.y)
    }
}

pub type ScreenCoord = i64;

#[derive(Debug, Clone, Copy)]
pub struct ScreenPos {
    pub x: ScreenCoord,
    pub y: ScreenCoord,
}

impl ScreenPos {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };

    pub fn new(x: ScreenCoord, y: ScreenCoord) -> Self {
        Self { x, y }
    }
}

impl Display for ScreenPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Pos> for ScreenPos {
    fn from(value: Pos) -> Self {
        Self {
            x: value.x as ScreenCoord,
            y: (value.y / 2.0) as ScreenCoord,
        }
    }
}

impl From<ScreenPos> for Pos {
    fn from(value: ScreenPos) -> Self {
        Self {
            x: value.x as Coord,
            y: (value.y * 2) as Coord,
        }
    }
}
