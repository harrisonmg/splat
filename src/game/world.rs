use std::time::Duration;

use crate::engine::{Camera, Input, Renderer};

use super::Stage;

/// Instead of getting deep into game engine design theory, just pass a big struct of mutable
/// world state around since this engine is single threaded.
pub struct World {
    pub update_interval: Duration,
    pub renderer: Renderer,
    pub camera: Camera,
    pub input: Input,
    pub stage: Stage,
}

impl World {
    pub fn update(&mut self) -> std::io::Result<()> {
        self.input.update(&self.camera)
    }
}
