use std::time::{Duration, Instant};

use crate::engine::{Drawable, Pos, Ray};

const LINK_TIME: Duration = Duration::from_millis(5);

pub struct Chain {
    pub ray: Ray,
    links: Vec<Pos>,
    num_links_out: usize,
    start_time: Option<Instant>,
}

impl Chain {
    pub fn new(ray: Ray) -> Self {
        let links = ray.march();
        Self {
            ray,
            links,
            num_links_out: 0,
            start_time: Some(Instant::now()),
        }
    }

    pub fn new_deployed(ray: Ray) -> Self {
        let links = ray.march();
        let num_links = links.len();
        Self {
            ray,
            links,
            num_links_out: num_links,
            start_time: None,
        }
    }

    pub fn update(&mut self) {
        if let Some(start_time) = self.start_time {
            let num_links_out = start_time.elapsed().as_secs_f64() / LINK_TIME.as_secs_f64();
            self.num_links_out = num_links_out as usize;
            if self.deployed() {
                self.start_time = None;
            }
        }
    }

    pub fn deployed(&self) -> bool {
        self.num_links_out >= self.links.len()
    }

    pub fn tangent(&self) -> Pos {
        let dir = self.ray.direction();
        Pos::new(dir.y, -dir.x)
    }
}

impl Drawable for Chain {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        for link in self.links.iter().take(self.num_links_out) {
            camera.paint_dot('•', *link, renderer);
        }
    }
}
