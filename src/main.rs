#![allow(dead_code)]

use std::{path::Path, time::Duration};

use crossterm::terminal;

use engine::{Animation, Button, Camera, Drawable, Input, Pos, Renderer, ScreenPos};
use game::{Border, Player, Stage, World};

mod engine;
mod game;

pub const UPDATE_INTERVAL: Duration = Duration::from_millis(10);

fn main() -> std::io::Result<()> {
    let size = terminal::window_size()?;
    let width = size.columns;
    let height = size.rows / 2;

    let renderer = Renderer::new(width, height)?;

    // leave room for border and status bar
    let camera = Camera {
        pos: Pos::ZERO,
        frame_pos: ScreenPos::new(1, 1),
        width: width - 2,
        height: height - 3,
    };

    let input = Input::new()?;
    let stage = Stage::load(Path::new("test.stage"))?;

    let mut world = World {
        update_interval: UPDATE_INTERVAL,
        renderer,
        camera,
        input,
        stage,
    };

    let border = Border;
    let mut player = Player::new();

    loop {
        world.renderer.clear();

        world.update()?;
        if world.input.pressed_this_frame(Button::Quit) {
            break;
        }

        player.update(&world);

        world.stage.draw(&world.camera, &mut world.renderer);

        world
            .camera
            .paint_sprite(anim.get_frame(), Pos::ZERO, &mut world.renderer);

        player.draw(&world.camera, &mut world.renderer);
        border.draw(&world.camera, &mut world.renderer);

        world.renderer.render()?;

        std::thread::sleep(world.update_interval);
    }

    Ok(())
}
