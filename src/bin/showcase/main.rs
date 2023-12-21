use crossterm::terminal;

use splat::engine::{Button, Camera, Drawable, Input, Logger, Pos, Renderer, ScreenPos};
use splat::game::{Bear, Border, UPDATE_RATE};
use ui_button::UiButtons;

mod ui_button;

fn main() -> std::io::Result<()> {
    let size = terminal::window_size()?;
    let width = size.columns;
    let height = size.rows - 10;

    let logger = Logger::setup().unwrap();
    let mut renderer = Renderer::new(width, height, Some(logger))?;

    // leave room for border and status bar
    let mut camera = Camera {
        pos: Pos::ZERO,
        frame_pos: ScreenPos::new(1, 1),
        width: width - 2,
        height: height - 3,
    };

    let ui_camera = Camera {
        pos: Pos::ZERO,
        frame_pos: ScreenPos::ZERO,
        width,
        height,
    };

    let mut input = Input::new()?;

    let border = Border;

    let mut ui_buttons = UiButtons::new();

    let mut bear = Bear::new(Pos::new(-50.0, 20.0));

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
        splat::dbg!(actual_rate);

        input.update(&camera)?;
        if input.pressed_this_frame(Button::Quit) {
            break;
        }
        splat::dbg!(input.mouse_pos);

        bear.update();
        ui_buttons.update(&input, &mut camera);

        renderer.clear();

        border.draw(&camera, &mut renderer);
        bear.draw(&camera, &mut renderer);
        ui_buttons.draw(&ui_camera, &mut renderer);

        renderer.render()?;

        loop_helper.loop_sleep();
    }

    Ok(())
}
