use std::io::Write;

use crossterm::{cursor, queue, style, terminal};

use crate::game::{Pos, ScreenCoord, ScreenPos};

pub type Dimension = u16;

pub struct Renderer {
    width: Dimension,
    height: Dimension,
    frame: Vec<Vec<char>>,
    stdout: std::io::Stdout,
}

impl Renderer {
    const CLEAR_CHAR: char = '-';

    pub fn new(width: Dimension, height: u16) -> Self {
        Self {
            width,
            height,
            frame: vec![vec![Self::CLEAR_CHAR; height as usize]; width as usize],
            stdout: std::io::stdout(),
        }
    }

    pub fn paint(&mut self, frame_x: Dimension, frame_y: Dimension, dot: char) {
        if frame_x < self.width && frame_y < self.height {
            self.frame[frame_x as usize][frame_y as usize] = dot;
        }
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.frame[x as usize][y as usize] = Self::CLEAR_CHAR;
            }
        }
    }

    pub fn render(&mut self) -> std::io::Result<()> {
        queue!(
            self.stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;

        for x in 0..self.width {
            for y in 0..self.height {
                let dot = self.frame[x as usize][y as usize];
                queue!(self.stdout, cursor::MoveTo(x, y), style::Print(dot))?
            }
        }

        self.stdout.flush()?;

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = queue!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Show,
        );
    }
}

pub type Sprite = Vec<Vec<char>>;

pub struct Camera {
    pub pos: Pos,
    pub width: Dimension,
    pub height: Dimension,
}

impl Camera {
    pub fn paint_sprite(&self, sprite: &Sprite, pos: Pos, renderer: &mut Renderer) {
        let sprite_pos = ScreenPos::from(pos - self.pos);
        let cam_pos = ScreenPos::from(self.pos);

        for y in sprite_pos.y..cam_pos.y + self.height as ScreenCoord {
            let sprite_y = (y - sprite_pos.y) as usize;
            if sprite_y >= sprite.len() {
                break;
            }

            for x in sprite_pos.x..cam_pos.x + self.width as ScreenCoord {
                let sprite_x = (x - sprite_pos.x) as usize;
                if sprite_x >= sprite[sprite_y].len() {
                    break;
                }

                let frame_x = x - cam_pos.x;
                let frame_y = y - cam_pos.y;

                if frame_x >= 0
                    && frame_x < self.width as ScreenCoord
                    && frame_y >= 0
                    && frame_y < self.height as ScreenCoord
                {
                    renderer.paint(
                        frame_x as Dimension,
                        frame_y as Dimension,
                        sprite[sprite_y][sprite_x],
                    );
                }
            }
        }
    }
}

pub trait Drawable {
    fn draw(camera: &Camera, renderer: &mut Renderer);
}
