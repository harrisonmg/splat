use crate::{
    debug,
    engine::{Button, Drawable, Pos, Ray, Sprite},
};

use super::{Rope, World};

pub struct Player {
    pos: Pos,
    sprite: Sprite,
    rope: Option<Rope>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Pos::ZERO,
            //sprite: vec![vec!['╭', '╮'], vec!['╰', '╯']],
            sprite: vec![vec!['█']],
            rope: None,
        }
    }

    pub fn update(&mut self, world: &World) {
        let speed = 50.0;

        if world.input.pressed(Button::RightMouse) {
            let diff = world.input.mouse_pos - self.pos;
            if diff.magnitude() > 0.0 {
                let full_step = diff
                    .normalize()
                    .scale(speed * world.update_interval.as_secs_f32());
                self.pos += if diff.magnitude() < full_step.magnitude() {
                    diff
                } else {
                    full_step
                };
            }
        }

        if world.input.pressed_this_frame(Button::LeftMouse) {
            self.rope = Some(Rope::new(Ray {
                start: self.pos,
                end: world.input.mouse_pos,
            }));
        } else if world.input.released(Button::LeftMouse) {
            self.rope = None;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        debug!(renderer, format!("player.pos: {:?}", self.pos));
        if let Some(rope) = self.rope {
            rope.draw(camera, renderer);
        }
        camera.paint_sprite(&self.sprite, self.pos, renderer);
    }
}
