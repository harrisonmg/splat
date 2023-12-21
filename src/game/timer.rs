use std::time::Instant;

use crate::engine::Drawable;

pub struct Timer {
    start_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
}

impl Drawable for Timer {
    fn draw(&self, _camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        let time_string = format!("{:.0?}", self.start_time.elapsed());
        let y = renderer.height() - 1;
        let x_offset = renderer.width() - time_string.len() as u16;
        for (i, dot) in time_string.chars().enumerate().take(time_string.len() - 1) {
            renderer.paint(i as u16 + x_offset, y, dot);
        }
    }
}
