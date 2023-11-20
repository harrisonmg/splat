use crate::{
    debug,
    engine::{Button, Drawable, Input, Pos, Ray, Sprite},
};

use super::{Chain, UPDATE_INTERVAL};

pub struct Player {
    pos: Pos,
    sprite: Sprite,
    chain: Option<Chain>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Pos::ZERO,
            //sprite: vec![vec!['╭', '╮'], vec!['╰', '╯']],
            sprite: vec![vec!['█']],
            chain: None,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let speed = 50.0;

        if input.pressed(Button::RightMouse) {
            let diff = input.mouse_pos - self.pos;
            if diff.magnitude() > 0.0 {
                let full_step = diff
                    .normalize()
                    .scale(speed * UPDATE_INTERVAL.as_secs_f32());
                self.pos += if diff.magnitude() < full_step.magnitude() {
                    diff
                } else {
                    full_step
                };
            }
        }

        if input.pressed_this_frame(Button::LeftMouse) {
            self.chain = Some(Chain::new(Ray {
                start: self.pos,
                end: input.mouse_pos,
            }));
        } else if input.released(Button::LeftMouse) {
            self.chain = None;
        } else if let Some(chain) = self.chain.as_mut() {
            chain.update();
        }
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::engine::Camera, renderer: &mut crate::engine::Renderer) {
        debug!(renderer, format!("player.pos: {:?}", self.pos));
        if let Some(chain) = self.chain.as_ref() {
            chain.draw(camera, renderer);
        }
        camera.paint_sprite(&self.sprite, self.pos, renderer);
    }
}
