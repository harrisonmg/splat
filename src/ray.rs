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
    /// Returns an iterator of positions starting from the ray start pos until the end, in
    /// increments of the smaller dimension of single screen dot.
    pub fn march(&self) -> impl Iterator<Item = Pos> {
        let heading = (self.end - self.start).normalize();
        let dot_size = Pos::from(ScreenPos::ONE);
        let step = heading.scale(dot_size.x.min(dot_size.y));

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
