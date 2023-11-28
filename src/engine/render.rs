use std::io::Write;

use crossterm::{cursor, queue, style, terminal};

use super::{Camera, Logger};

pub type Dimension = u16;

pub struct Renderer {
    width: Dimension,
    height: Dimension,
    frame: Vec<Vec<char>>,
    stdout: std::io::Stdout,
    logger: Option<&'static Logger>,
}

impl Renderer {
    const CLEAR_CHAR: char = ' ';

    pub fn new(
        width: Dimension,
        height: Dimension,
        logger: Option<&'static Logger>,
    ) -> std::io::Result<Self> {
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;

        if let Some(logger) = logger.as_ref() {
            logger.drain();
        }

        Ok(Self {
            width,
            height,
            frame: vec![vec![Self::CLEAR_CHAR; width as usize]; height as usize],
            stdout,
            logger,
        })
    }

    pub fn width(&self) -> Dimension {
        self.width
    }

    pub fn height(&self) -> Dimension {
        self.height
    }

    pub fn paint(&mut self, frame_x: Dimension, frame_y: Dimension, dot: char) {
        if frame_x < self.width && frame_y < self.height {
            self.frame[frame_y as usize][frame_x as usize] = dot;
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.frame[y as usize][x as usize] = Self::CLEAR_CHAR;
            }
        }
    }

    pub fn render(&mut self) -> std::io::Result<()> {
        queue!(self.stdout, cursor::MoveTo(0, 0))?;

        for x in 0..self.height {
            let line = String::from_iter(self.frame[x as usize].iter());
            queue!(self.stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        if let Some(logger) = self.logger {
            for msg in logger.drain() {
                queue!(
                    self.stdout,
                    style::Print(msg),
                    terminal::Clear(terminal::ClearType::UntilNewLine),
                    cursor::MoveToNextLine(1),
                )?;
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

pub trait Drawable {
    fn draw(&self, camera: &Camera, renderer: &mut Renderer);
}
