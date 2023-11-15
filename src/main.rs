#![allow(dead_code)]

use game::{Pos, HEIGHT, UPDATE_INTERVAL, WIDTH};
use input::Input;
use render::{Camera, Renderer};

use crate::input::Button;

mod game;
mod input;
mod render;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let camera = Camera {
        pos: Pos::new(5.0, 5.0),
        width: WIDTH,
        height: HEIGHT,
    };

    let mut input = Input::new()?;

    loop {
        renderer.clear();

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        debug!(renderer, format!("{:?}", input.mouse_pos));

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
