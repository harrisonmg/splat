use std::time::{Duration, Instant};

use crate::engine::{Drawable, Pos, Ray};

const LINK_TIME: Duration = Duration::from_millis(5);

enum State {
    Deploying,
    Deployed,
    Retracting,
    Retracted,
}

pub struct Chain {
    pub ray: Ray,
    state: State,
    links: Vec<Pos>,
    num_links_out: usize,
    start_time: Instant,
    just_deployed: bool,
}

impl Chain {
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            state: State::Retracted,
            links: Vec::new(),
            num_links_out: 0,
            start_time: Instant::now(),
            just_deployed: false,
        }
    }

    pub fn update(&mut self) {
        self.just_deployed = false;
        self.links = self.ray.march();

        match self.state {
            State::Deploying => {
                let num_links_out =
                    self.start_time.elapsed().as_secs_f64() / LINK_TIME.as_secs_f64();
                let num_links_out = (num_links_out as usize).min(self.links.len());
                self.num_links_out = num_links_out;

                if self.num_links_out == self.links.len() {
                    self.state = State::Deployed;
                }
            }
            State::Retracting => {
                if self.num_links_out == 0 {
                    self.state = State::Retracted;
                } else if self.start_time.elapsed() >= LINK_TIME {
                    self.num_links_out -= 1;
                }
            }
            _ => (),
        }
    }

    pub fn deploy(&mut self) {
        self.state = State::Deploying;
        self.start_time = Instant::now();
        self.just_deployed = true;
    }

    pub fn deployed(&self) -> bool {
        matches!(self.state, State::Deploying | State::Deployed)
    }

    pub fn just_deployed(&self) -> bool {
        self.just_deployed
    }

    pub fn retract(&mut self) {
        self.state = State::Retracting;
        self.start_time = Instant::now();
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
