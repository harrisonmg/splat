#![allow(dead_code)]

use std::path::Path;

use crossterm::terminal;

use engine::{Button, Camera, Coord, Drawable, Input, Logger, Pos, Renderer, ScreenPos};
use game::{Border, Player, Stage, UPDATE_RATE};

mod engine;
mod game;

fn main() -> std::io::Result<()> {
    let size = terminal::window_size()?;
    let width = size.columns;
    let height = size.rows / 2;

    let logger = Logger::setup().unwrap();
    let mut renderer = Renderer::new(width, height, Some(logger))?;

    // leave room for border and status bar
    let mut camera = Camera {
        pos: Pos::ZERO,
        frame_pos: ScreenPos::new(1, 1),
        width: width - 2,
        height: height - 3,
    };

    let mut input = Input::new()?;
    let stage = Stage::load(Path::new("test.stage"))?;

    let border = Border;
    let mut player = Player::new();
    player.pos.x = width as Coord / 2.0;

    // use spin_sleep since native sleep is often too slow / low res
    let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(UPDATE_RATE);

    loop {
        loop_helper.loop_start();

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        player.update(&input, &stage);
        camera.update(&player, &input);

        renderer.clear();

        stage.draw(&camera, &mut renderer);
        player.draw(&camera, &mut renderer);
        border.draw(&camera, &mut renderer);

        renderer.render()?;

        loop_helper.loop_sleep();
    }

    Ok(())
}

#[macro_export]
macro_rules! dbg {
    ($val: expr) => {
        log::debug!("{:?} = {:?}", stringify!($val), $val)
    };
}
