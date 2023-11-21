use super::{Dimension, Pos, Renderer, ScreenCoord, ScreenPos, Sprite};

pub struct Camera {
    pub pos: Pos,
    pub frame_pos: ScreenPos,
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
                    let dot = sprite[sprite_y][sprite_x];
                    if dot.is_whitespace() {
                        continue;
                    }

                    renderer.paint(
                        (self.frame_pos.x + frame_x) as Dimension,
                        (self.frame_pos.y + frame_y) as Dimension,
                        dot,
                    );
                }
            }
        }
    }

    pub fn paint_dot(&self, dot: char, pos: Pos, renderer: &mut Renderer) {
        let dot_pos = ScreenPos::from(pos - self.pos);
        let cam_pos = ScreenPos::from(self.pos);

        if dot_pos.x >= 0
            && dot_pos.x < cam_pos.x + self.width as ScreenCoord
            && dot_pos.y >= 0
            && dot_pos.y < cam_pos.y + self.height as ScreenCoord
        {
            renderer.paint(
                (self.frame_pos.x + dot_pos.x) as Dimension,
                (self.frame_pos.y + dot_pos.y) as Dimension,
                dot,
            );
        }
    }
}
