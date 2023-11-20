use std::time::Duration;

use crate::engine::{Animation, Drawable, Ray};

const ANIM_FRAME_TIME: Duration = Duration::from_millis(100);

pub struct Rope {
    ray: Ray,
    anim: Animation,
}

impl Rope {
    pub fn new(ray: Ray) -> Self {
        todo!()
    }
}

impl Drawable for Rope {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        todo!()
    }
}
