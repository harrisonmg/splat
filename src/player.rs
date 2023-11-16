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
    ray: Option<Ray>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Pos {
                x: WIDTH as Coord / 2.0 - 2.0,
                y: HEIGHT as Coord / 2.0 - 1.0,
            },
            sprite: vec![vec!['╭', '╮'], vec!['╰', '╯']],
            ray: None,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let speed = 50.0;

        if input.pressed(Button::RightMouse) {
            let diff = input.mouse_pos - self.pos;
            if diff.magnitude() > 1.0 {
                self.pos += diff
                    .normalize()
                    .scale(speed * UPDATE_INTERVAL.as_secs_f32());
            }
        }

        self.ray = Some(Ray {
            start: self.pos,
            end: input.mouse_pos,
        });
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::render::Camera, renderer: &mut crate::render::Renderer) {
        camera.paint_sprite(&self.sprite, self.pos, renderer);
        debug!(renderer, format!("player.pos: {:?}", self.pos));
        if let Some(ray) = self.ray.as_ref() {
            ray.draw(camera, renderer);
        }
    }
}
