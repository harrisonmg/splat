use log::debug;

use crate::engine::{Button, Coord, Drawable, Input, Pos, Ray, Signed};

use super::{Chain, UPDATE_INTERVAL};

pub const GRAVITY: Coord = 100.0;
pub const AIR_DRAG: Coord = 0.0005;
pub const SWING_KICK: Coord = 20.0;

pub struct Player {
    pub pos: Pos,
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
        // for testing
        if input.pressed(Button::RightMouse) {
            self.pos = input.mouse_pos;
            self.vel = Pos::ZERO;
        }

        // chain make and break
        if input.pressed_this_frame(Button::LeftMouse) && self.pos != input.mouse_pos {
            let chain = Chain::new(Ray {
                start: self.pos,
                end: input.mouse_pos,
            });
            self.chain = Some(chain);
        } else if input.released(Button::LeftMouse) {
            self.chain = None;
        } else if let Some(chain) = self.chain.as_mut() {
            chain.update();
        }

        let grav_force = Pos::new(0.0, GRAVITY);

        let drag_mag = self.vel.magnitude().powi(2) * AIR_DRAG;
        let drag_force = if drag_mag > 0.0 {
            self.vel.normalize().scale(-drag_mag)
        } else {
            Pos::ZERO
        };

        let chain_force = if let Some(chain) = self.chain.as_ref() {
            if !chain.deployed() {
                // pause in air while deploying
                return;
            }

            // transform the velocity into the basis of the swing tangent
            let tangent = chain.tangent();
            let vel_trans = self.vel.transform_basis(tangent);

            let kick_force = if chain.just_deployed() {
                // give a little kick when starting a swing
                // if gravity is in your favor
                let grav_dir = Pos::new(0.0, 1.0).transform_basis(tangent).x.sign();
                let vel_dir = vel_trans.x.sign();

                if grav_dir == vel_dir {
                    tangent.scale(SWING_KICK * vel_dir / UPDATE_INTERVAL.as_secs_f32())
                } else {
                    Pos::ZERO
                }
            } else {
                Pos::ZERO
            };

            // chain is actually a super stiff spring
            // find the component of player velocity that would stretch the chain
            let vel_opposite_chain = -vel_trans.y;

            // calculate the force to negate that velocity in one step
            let spring_mag = vel_opposite_chain / UPDATE_INTERVAL.as_secs_f32();

            // and the actual pendulum tension force
            let chain_mag = -GRAVITY * chain.ray.angle().sin();

            let chain_force = chain.ray.direction().scale(spring_mag + chain_mag);

            chain_force + kick_force
        } else {
            Pos::ZERO
        };

        let total_force = grav_force + drag_force + chain_force;

        self.vel += total_force.scale(UPDATE_INTERVAL.as_secs_f32());
        let new_pos = self.pos + self.vel.scale(UPDATE_INTERVAL.as_secs_f32());

        // TODO collision check with ray
        self.pos = new_pos;

        if let Some(chain) = self.chain.as_mut() {
            let chain_len = chain.ray.length();
            let new_dir = (self.pos - chain.ray.end).normalize();
            self.pos = chain.ray.end + new_dir.scale(chain_len);
            *chain = Chain::new_deployed(Ray {
                start: self.pos,
                end: chain.ray.end,
            });
        }

        // TODO stage death
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
