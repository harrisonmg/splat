use log::debug;

use crate::engine::{Button, Coord, Drawable, Input, Pos, Ray, Signed};

use super::{Chain, UPDATE_INTERVAL};

pub const GRAVITY: Coord = 300.0;
pub const AIR_DRAG: Coord = 0.00;

pub struct Player {
    pos: Pos,
    vel: Pos,
    chain: Option<Chain>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Pos::ZERO,
            vel: Pos::ZERO,
            chain: None,
        }
    }

    pub fn update(&mut self, input: &Input) {
        if input.pressed(Button::RightMouse) {
            self.pos = input.mouse_pos;
            self.vel = Pos::ZERO;
        }

        if input.pressed_this_frame(Button::LeftMouse) && self.pos != input.mouse_pos {
            self.chain = Some(Chain::new(Ray {
                start: self.pos,
                end: input.mouse_pos,
            }));
        } else if input.released(Button::LeftMouse) {
            self.chain = None;
        } else if let Some(chain) = self.chain.as_mut() {
            chain.update();
            //if !chain.deployed() {
            //    return;
            //}
        }

        let gravity_force = Pos::new(0.0, GRAVITY);

        let drag_mag = self.vel.magnitude().powi(2) * AIR_DRAG;
        let drag_force = if drag_mag > 0.0 {
            self.vel.normalize().scale(-drag_mag)
        } else {
            Pos::ZERO
        };

        let chain_force = if let Some(chain) = self.chain.as_ref() {
            let tmp = chain.ray.direction().scale(-GRAVITY);
            Pos::new(tmp.y, -tmp.x)
        } else {
            Pos::ZERO
        };

        let total_force = gravity_force + drag_force + chain_force;
        debug!("gravity {gravity_force}");
        debug!("drag {drag_force}");
        debug!("chain {chain_force}");
        debug!("total {total_force}");

        self.vel += total_force.scale(UPDATE_INTERVAL.as_secs_f32());
        self.pos += self.vel.scale(UPDATE_INTERVAL.as_secs_f32());

        if let Some(chain) = self.chain.as_mut() {
            let chain_len = chain.ray.length();
            let new_dir = (self.pos - chain.ray.end).normalize();
            self.pos = chain.ray.end + new_dir.scale(chain_len);
            *chain = Chain::new_deployed(Ray {
                start: self.pos,
                end: chain.ray.end,
            });
        }

        // TODO
        if self.pos.y > 50.0 {
            self.pos.y = 0.0;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        if let Some(chain) = self.chain.as_ref() {
            chain.draw(camera, renderer);
        }
        camera.paint_dot('O', self.pos, renderer);
    }
}
