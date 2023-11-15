use crate::{
    game::{Coord, Pos, HEIGHT, UPDATE_INTERVAL, WIDTH},
    input::{Button, Input},
    render::{Drawable, Sprite},
};

pub struct Player {
    pos: Pos,
    sprite: Sprite,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Pos {
                x: WIDTH as Coord / 2.0 - 2.0,
                y: HEIGHT as Coord / 2.0 - 1.0,
            },
            sprite: vec![vec!['╭', '╮'], vec!['╰', '╯']],
        }
    }

    pub fn update(&mut self, input: &Input) {
        let speed = 50.0;

        if input.pressed(Button::RightMouse) {
            self.pos += speed * UPDATE_INTERVAL.as_secs_f32();
        }
    }
}

impl Drawable for Player {
    fn draw(&self, camera: &crate::render::Camera, renderer: &mut crate::render::Renderer) {
        camera.paint_sprite(&self.sprite, self.pos, renderer);
    }
}
