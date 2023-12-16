use crate::engine::{Camera, ScreenPos};

use super::Player;

impl Camera {
    pub fn update(&mut self, player: &Player) {
        // quantize to screen coords to avoid collision alignment looking weird
        let offset = ScreenPos::new((self.width / 2).into(), (self.height / 2).into());
        let quantized = ScreenPos::from(player.pos) - offset;
        self.pos = quantized.into();
    }
}
