#![allow(dead_code)]

use std::path::Path;

use crossterm::terminal;

use engine::{Button, Camera, Drawable, Input, Pos, Renderer, ScreenPos};
use game::{Border, Player, Stage, UPDATE_INTERVAL};

mod engine;
mod game;

fn main() -> std::io::Result<()> {
    let size = terminal::window_size()?;
    let width = size.columns;
    let height = size.rows / 2;

    let mut renderer = Renderer::new(width, height)?;

    // leave room for border and status bar
    let camera = Camera {
        pos: Pos::ZERO,
        frame_pos: ScreenPos::new(1, 1),
        width: width - 2,
        height: height - 3,
    };

    let mut input = Input::new()?;
    let stage = Stage::load(Path::new("test.stage"))?;

    let border = Border;
    let mut player = Player::new();

    loop {
        renderer.clear();

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        player.update(&input);

        stage.draw(&camera, &mut renderer);

        player.draw(&camera, &mut renderer);
        border.draw(&camera, &mut renderer);

        renderer.render()?;

        std::thread::sleep(UPDATE_INTERVAL);
    }

    Ok(())
}
