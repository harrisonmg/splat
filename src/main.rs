use game::{Pos, HEIGHT, UPDATE_INTERVAL, WIDTH};
use render::{Camera, Renderer};

mod game;
mod render;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let camera = Camera {
        pos: Pos::ZERO,
        width: WIDTH,
        height: HEIGHT,
    };

    let sprite = vec![vec!['1', '2'], vec!['3', '4']];
    let mut pos = Pos::new(WIDTH as i64 / 2, HEIGHT as i64 / 2);

    loop {
        renderer.clear();

        camera.paint_sprite(&sprite, pos, &mut renderer);

        pos.x += 2;
        pos.y += 1;

        renderer.render()?;

        std::thread::sleep(UPDATE_INTERVAL);
    }
}
