#![allow(dead_code)]

use std::path::Path;

use border::Border;
use game::{Pos, ScreenPos, HEIGHT, UPDATE_INTERVAL, WIDTH};
use input::Input;
use player::Player;
use render::{Camera, Drawable, Renderer};
use stage::Stage;

use crate::input::Button;

mod border;
mod game;
mod input;
mod player;
mod ray;
mod render;
mod stage;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    // leave room for border and status bar
    let camera = Camera {
        pos: Pos::ZERO,
        frame_pos: ScreenPos::new(1, 1),
        width: WIDTH - 2,
        height: HEIGHT - 3,
    };

    let border = Border;

    let mut input = Input::new()?;

    let stage = Stage::load(&Path::new("test.stage"))?;

    let mut player = Player::new();

    loop {
        renderer.clear();

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        player.update(&input);

        debug!(renderer, format!("input.mouse_pos: {:?}", input.mouse_pos));

        stage.draw(&camera, &mut renderer);
        player.draw(&camera, &mut renderer);
        border.draw(&camera, &mut renderer);

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
