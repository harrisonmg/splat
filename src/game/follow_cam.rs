use crate::engine::{Camera, Coord, Input};

use super::Player;

impl Camera {
    pub fn update(&mut self, player: &Player, input: &Input) {
        self.pos.x = player.pos.x - Coord::from(self.width) / 2.0;
        self.pos.x = input.mouse_pos.x - Coord::from(self.width) / 2.0;
        //self.pos.y = player.pos.y - Coord::from(self.height) / 2.0;
    }
}
