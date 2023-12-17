use crate::engine::{Pos, ScreenPos};

use super::Coord;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub start: Pos,
    pub end: Pos,
}

impl Ray {
    /// Returns a vec of positions starting from the ray start pos until the end, in
    /// increments of the smaller dimension of single screen dot.
    pub fn march(&self) -> Vec<Pos> {
        let heading = (self.end - self.start).normalize();
        let dot_size = Pos::from(ScreenPos::ONE);
        let step_size = dot_size.x.min(dot_size.y);
        let step = heading.scale(step_size);

        let mut pos = self.start;
        let mut path = vec![pos];
        let full_mag = (self.end - self.start).magnitude();

        while (pos - self.start).magnitude() < full_mag {
            pos += step;
            path.push(pos);
        }

        if let Some(last) = path.last_mut() {
            *last = self.end;
        }

        path
    }

    pub fn direction(&self) -> Pos {
        (self.end - self.start).normalize()
    }

    pub fn angle(&self) -> Coord {
        let dir = self.direction();
        dir.y.atan2(dir.x)
    }

    pub fn length(&self) -> Coord {
        (self.end - self.start).magnitude()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InifiniteRay {
    pos: Pos,
    direction: Pos,
}

impl InifiniteRay {
    pub fn new(start: Pos, direction: Pos) -> Self {
        Self {
            pos: start,
            direction: direction.normalize(),
        }
    }
}

impl Iterator for InifiniteRay {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let dot_size = Pos::from(ScreenPos::ONE);
        let step_size = dot_size.x.min(dot_size.y);
        self.pos += self.direction.scale(step_size);
        Some(self.pos)
    }
}
