use crate::render::Drawable;

pub struct Border;

impl Border {
    const TOP_LEFT_CORNER: char = '┌';
    const TOP_RIGHT_CORNER: char = '┐';
    const BOTTOM_LEFT_CORNER: char = '└';
    const BOTTOM_RIGHT_CORNER: char = '┘';
    const VERTICAL: char = '│';
    const HORIZONTAL: char = '─';
}

impl Drawable for Border {
    fn draw(&self, camera: &crate::render::Camera, renderer: &mut crate::render::Renderer) {
        let width = camera.width + 1;
        let height = camera.height + 1;

        renderer.paint(0, 0, Self::TOP_LEFT_CORNER);

        for x in 1..width {
            renderer.paint(x, 0, Self::HORIZONTAL);
        }

        renderer.paint(width, 0, Self::TOP_RIGHT_CORNER);

        for y in 1..height {
            renderer.paint(width, y, Self::VERTICAL);
        }

        renderer.paint(width, height, Self::BOTTOM_RIGHT_CORNER);

        for x in 1..width {
            renderer.paint(x, height, Self::HORIZONTAL);
        }

        renderer.paint(0, height, Self::BOTTOM_LEFT_CORNER);

        for y in 1..height {
            renderer.paint(0, y, Self::VERTICAL);
        }
    }
}
