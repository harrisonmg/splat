use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::engine::{Drawable, Pos, ScreenPos};

pub enum Tile {
    Something,
    Nothing,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Nothing,
            _ => Self::Something,
        }
    }
}

pub struct Stage {
    data: Vec<Vec<char>>,
}

impl Stage {
    pub fn load(stage_file: &Path) -> std::io::Result<Self> {
        let mut data = Vec::new();
        let file = File::open(stage_file)?;
        for line in BufReader::new(file).lines() {
            data.push(line?.chars().collect());
        }
        Ok(Self { data })
    }

    pub fn check_pos(&self, pos: Pos) -> Tile {
        let (x, y) = ScreenPos::from(pos).indices();
        self.data
            .get(y)
            .and_then(|row| row.get(x))
            .map_or(Tile::Nothing, |dot| Tile::from(*dot))
    }
}

impl Drawable for Stage {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        camera.paint_sprite(&self.data, Pos::ZERO, renderer);
    }
}
