use crate::{
    debug,
    game::{Pos, ScreenPos},
    render::Drawable,
};

pub struct Ray {
    pub start: Pos,
    pub end: Pos,
}

impl Ray {
    /// Returns an iterator of screen positions starting from the ray start pos until the end
    pub fn march(&self) -> impl Iterator<Item = ScreenPos> {
        let heading = self.end - self.start;
        let step = heading.normalize();
        let mut pos = self.start;
        let mut path = vec![pos.into()];
        let full_mag = (self.end - self.start).magnitude();
        while (pos - self.start).magnitude() < full_mag {
            pos += step;
            path.push(pos.into());
        }
        path.into_iter()
    }
}

impl Drawable for Ray {
    fn draw(&self, camera: &crate::render::Camera, renderer: &mut crate::render::Renderer) {
        for step in self.march() {
            camera.paint_dot('*', step.into(), renderer);
            debug!(renderer, format!("ray step: {:?}", step))
        }
    }
}
