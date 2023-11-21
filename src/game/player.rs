use log::debug;

use crate::engine::{Button, Coord, Drawable, Input, Pos, Ray, Signed};

use super::{Chain, UPDATE_INTERVAL};

pub const GRAVITY: Pos = Pos::new(0.0, 200.0);
pub const AIR_DRAG: Coord = 0.01;

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
        if input.pressed_this_frame(Button::LeftMouse) {
            self.chain = Some(Chain::new(Ray {
                start: self.pos,
                end: input.mouse_pos,
            }));
        } else if input.released(Button::LeftMouse) {
            self.chain = None;
        } else if let Some(chain) = self.chain.as_mut() {
            if chain.deployed() {
                *chain = Chain::new_deployed(Ray {
                    start: self.pos,
                    end: chain.ray.end,
                });
            } else {
                chain.update();
            }
        }

        self.vel += GRAVITY.scale(UPDATE_INTERVAL.as_secs_f32());

        let drag_mag = self.vel.magnitude().powi(2) * AIR_DRAG;
        let drag = self.vel.normalize().scale(-drag_mag);
        debug!("drag {drag}");
        self.vel += drag.scale(UPDATE_INTERVAL.as_secs_f32());

        if input.pressed(Button::RightMouse) {
            self.pos = input.mouse_pos;
            self.vel = Pos::ZERO;
        } else if let Some(chain) = self.chain.as_ref() {
            let chain_dir = chain.ray.direction();
            let chain_tan = Pos::new(chain_dir.y, -chain_dir.x);

            let vel_trans = Pos::new(
                chain_dir.x * self.vel.x + chain_tan.x * self.vel.y,
                chain_dir.y * self.vel.x + chain_tan.y * self.vel.y,
            );

            let vel_dir = vel_trans.y.sign();
            self.vel = chain_tan.scale(self.vel.magnitude() * vel_dir);
            debug!("chain vel {}", self.vel);
        }

        self.pos += self.vel.scale(UPDATE_INTERVAL.as_secs_f32());
        debug!("pos {}", self.pos);

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
