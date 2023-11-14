use game::{Coord, Pos, HEIGHT, UPDATE_INTERVAL, WIDTH};
use render::{Camera, Renderer};

mod game;
mod render;

fn main() -> anyhow::Result<()> {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let camera = Camera {
        pos: Pos::ZERO,
        width: WIDTH,
        height: HEIGHT,
    };

    let sprite = vec![vec!['1', '2'], vec!['3', '4']];
    let mut pos = Pos::new(WIDTH as Coord / 4.0, HEIGHT as Coord / 4.0);

    loop {
        renderer.clear();

        camera.paint_sprite(&sprite, pos, &mut renderer);

        pos.x += 1.0;
        pos.y += 0.5;

        renderer.render()?;

        std::thread::sleep(UPDATE_INTERVAL);
    }
}
