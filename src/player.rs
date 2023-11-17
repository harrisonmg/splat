use crate::{
    debug,
    game::{Coord, Pos, HEIGHT, UPDATE_INTERVAL, WIDTH},
    input::{Button, Input},
    ray::Ray,
    render::{Drawable, Sprite},
};

pub struct Player {
    pos: Pos,
    sprite: Sprite,
    ray: Ray,
}

impl Player {
    pub fn new() -> Self {
        let pos = Pos {
            x: WIDTH as Coord / 2.0 - 2.0,
            y: HEIGHT as Coord / 2.0 - 1.0,
        };
        Self {
            pos,
            //sprite: vec![vec!['╭', '╮'], vec!['╰', '╯']],
            sprite: vec![vec!['█']],
            ray: Ray {
                start: pos,
                end: pos,
            },
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

        self.ray = Ray {
            start: self.pos,
            end: input.mouse_pos,
        };
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::render::Camera, renderer: &mut crate::render::Renderer) {
        debug!(renderer, format!("player.pos: {:?}", self.pos));
        self.ray.draw(camera, renderer);
        camera.paint_sprite(&self.sprite, self.pos, renderer);
    }
}
