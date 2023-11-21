use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::engine::{Drawable, Pos};

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
}

impl Drawable for Stage {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        camera.paint_sprite(&self.data, Pos::ZERO, renderer);
    }
}