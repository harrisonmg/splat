use super::{Dimension, Pos, Renderer, ScreenCoord, ScreenPos, Sprite};

pub struct Camera {
    pub pos: Pos,
    pub frame_pos: ScreenPos,
    pub width: Dimension,
    pub height: Dimension,
}

impl Camera {
    pub fn paint_sprite(&self, sprite: &Sprite, pos: Pos, renderer: &mut Renderer) {
        let cam_screen_pos = ScreenPos::from(self.pos);
        let sprite_screen_pos = ScreenPos::from(pos) - cam_screen_pos;

        let start_y = sprite_screen_pos.y.max(cam_screen_pos.y);
        let start_x = sprite_screen_pos.x.max(cam_screen_pos.x);

        let cam_screen_end = cam_screen_pos + ScreenPos::new(self.width.into(), self.height.into());
        let end_y = cam_screen_end
            .y
            .min(sprite_screen_pos.y + sprite.len() as ScreenCoord);

        crate::dbg!(self.width);
        crate::dbg!(self.height);
        crate::dbg!(sprite_screen_pos);
        crate::dbg!(cam_screen_pos);
        crate::dbg!(start_x);
        crate::dbg!(start_y);
        crate::dbg!(end_y);

        for y in start_y..end_y {
            let sprite_y = (y - start_y) as usize;
            let end_x = cam_screen_end
                .x
                .min(sprite_screen_pos.x + sprite[sprite_y].len() as ScreenCoord);
            crate::dbg!(end_x);

            for x in start_x..end_x {
                let sprite_x = (x - start_x) as usize;
                let dot = sprite[sprite_y][sprite_x];

                renderer.paint(
                    (self.frame_pos.x + x) as Dimension,
                    (self.frame_pos.y + y) as Dimension,
                    dot,
                );
            }
        }
    }

    pub fn paint_dot(&self, dot: char, pos: Pos, renderer: &mut Renderer) {
        let dot_screen_pos = ScreenPos::from(pos - self.pos);
        let cam_screen_pos = ScreenPos::from(self.pos);

        if dot_screen_pos.x >= 0
            && dot_screen_pos.x < cam_screen_pos.x + self.width as ScreenCoord
            && dot_screen_pos.y >= 0
            && dot_screen_pos.y < cam_screen_pos.y + self.height as ScreenCoord
        {
            renderer.paint(
                (self.frame_pos.x + dot_screen_pos.x) as Dimension,
                (self.frame_pos.y + dot_screen_pos.y) as Dimension,
                dot,
            );
        }
    }
}
