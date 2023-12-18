use std::path::Path;

use crossterm::terminal;

use splat::engine::{Button, Camera, Coord, Drawable, Input, Logger, Pos, Renderer, ScreenPos};
use splat::game::{Border, Player, Stage, UPDATE_RATE};

fn main() -> std::io::Result<()> {
    let size = terminal::window_size()?;
    let width = size.columns;
    let height = size.rows;

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
    let mut loop_helper = spin_sleep::LoopHelper::builder()
        .report_interval_s(1.0)
        .build_with_target_rate(UPDATE_RATE);
    let mut actual_rate = 0.0;

    loop {
        loop_helper.loop_start();

        if let Some(rate) = loop_helper.report_rate() {
            actual_rate = rate;
        }
        dbg!(actual_rate);

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }

        player.update(&input, &stage);
        camera.update(&player);

        renderer.clear();

        stage.draw(&camera, &mut renderer);
        player.draw(&camera, &mut renderer);
        border.draw(&camera, &mut renderer);

        renderer.render()?;

        loop_helper.loop_sleep();
    }

    Ok(())
}
