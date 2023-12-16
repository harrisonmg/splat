use crate::engine::{Button, Coord, Drawable, Input, Pos, Ray, Signed};

use super::{Chain, Stage, Tile, DELTA_TIME};

pub const GRAVITY: Coord = 100.0;
pub const AIR_DRAG: Coord = 0.01;
pub const SWING_KICK: Coord = 30.0;
pub const SPRING_KICK: Coord = 1.2;
pub const MIN_SPRING_VEL: Coord = 50.0;
pub const JUMP_VEL: Coord = 60.0;

pub struct Player {
    pub pos: Pos,
    vel: Pos,
    chain: Chain,
    stuck: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Pos::ZERO,
            vel: Pos::ZERO,
            chain: Chain::new(Ray {
                start: Pos::ZERO,
                end: Pos::ZERO,
            }),
            stuck: true,
        }
    }

    pub fn update(&mut self, input: &Input, stage: &Stage) {
        self.jump(input);
        self.chain_throw(input);
        let new_pos = self.kinematics();
        self.collision(new_pos, stage);
        self.chain.ray.start = self.pos;
    }

    fn jump(&mut self, input: &Input) {
        if self.stuck && input.pressed_this_frame(Button::RightMouse) {
            self.stuck = false;
            let jump_dir = (input.mouse_pos - self.pos).normalize();
            self.vel = jump_dir.scale(JUMP_VEL);
        }
    }

    fn chain_throw(&mut self, input: &Input) {
        if input.pressed_this_frame(Button::LeftMouse) && self.pos != input.mouse_pos {
            self.stuck = false;
            self.chain.ray.end = input.mouse_pos;
            self.chain.deploy();
        } else if input.released_this_frame(Button::LeftMouse) {
            self.chain.retract();
        } else {
            self.chain.update();
        }
    }

    fn kinematics(&mut self) -> Pos {
        // gravity
        let grav_force = if self.stuck {
            Pos::ZERO
        } else {
            Pos::new(0.0, GRAVITY)
        };

        // air drag
        let drag_mag = self.vel.magnitude().powi(2) * AIR_DRAG;
        let drag_force = if drag_mag > 0.0 {
            self.vel.normalize().scale(-drag_mag)
        } else {
            Pos::ZERO
        };

        // chain forces
        let chain_force = if self.chain.deployed() {
            // transform the velocity into the basis of the swing tangent
            let tangent = self.chain.tangent();
            let vel_trans = self.vel.transform_basis(tangent);

            let kick_force = if self.chain.just_deployed() {
                // give a little kick when starting a swing
                // if gravity is in your favor
                let grav_dir = Pos::new(0.0, 1.0).transform_basis(tangent).x.sign();
                let vel_dir = vel_trans.x.sign();

                if grav_dir == vel_dir {
                    tangent.scale(SWING_KICK * vel_dir / DELTA_TIME)
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
            let spring_mag = vel_opposite_chain / DELTA_TIME;

            // and the actual pendulum tension force
            let chain_mag = -GRAVITY * self.chain.ray.angle().sin();

            let chain_force = self.chain.ray.direction().scale(spring_mag + chain_mag);

            chain_force + kick_force
        } else {
            Pos::ZERO
        };

        let total_force = grav_force + drag_force + chain_force;

        self.vel += total_force.scale(DELTA_TIME);
        self.pos + self.vel.scale(DELTA_TIME)
    }

    fn collision(&mut self, new_pos: Pos, stage: &Stage) {
        let traj = Ray {
            start: self.pos,
            end: new_pos,
        };
        let steps = traj.march();

        for i in 1..steps.len() {
            let next_step = steps[i];
            match stage.check_pos(next_step) {
                Tile::Nothing => {
                    self.pos = next_step;
                    continue;
                }
                Tile::Something => {
                    self.vel = Pos::ZERO;
                    self.stuck = true;
                }
                Tile::Spring => {
                    self.vel.y *= -SPRING_KICK;
                    self.vel.y = self.vel.y.min(-MIN_SPRING_VEL);
                }
            }
        }
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        self.chain.draw(camera, renderer);
        camera.paint_dot('O', self.pos, renderer);
    }
}
