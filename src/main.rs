#![allow(dead_code)]

use game::{Pos, HEIGHT, UPDATE_INTERVAL, WIDTH};
use input::Input;
use player::Player;
use render::{Camera, Drawable, Renderer};

use crate::input::Button;

mod game;
mod input;
mod player;
mod render;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let camera = Camera {
        pos: Pos::ZERO,
        width: WIDTH,
        height: HEIGHT,
    };

    let mut input = Input::new()?;

    let mut player = Player::new();

    loop {
        renderer.clear();

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        player.update(&input);

        player.draw(&camera, &mut renderer);

        renderer.render()?;

        std::thread::sleep(UPDATE_INTERVAL);
    }

    Ok(())
}

#[macro_export]
macro_rules! debug {
    ($renderer: ident, $msg: expr) => {
        if cfg!(debug_assertions) {
            $renderer.debug($msg);
        }
    };
}
