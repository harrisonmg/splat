use std::time::Duration;

use crate::engine::{Animation, Drawable, Ray};

const ANIM_FRAME_TIME: Duration = Duration::from_millis(100);

pub struct Chain {
    ray: Ray,
    anim: Animation,
}

impl Chain {
    pub fn new(ray: Ray) -> Self {
        todo!()
    }

    pub fn update(&mut self) {}
}

impl Drawable for Chain {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        todo!()
    }
}
