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
        pos: Pos::ZERO,
        width: WIDTH,
        height: HEIGHT,
    };

    let mut input = Input::new()?;

    loop {
        renderer.clear();

        input.update()?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        renderer.render()?;
        println!("{:#?}", input.state);

        std::thread::sleep(UPDATE_INTERVAL);
    }

    Ok(())
}
