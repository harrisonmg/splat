use crate::engine::{Camera, Coord, Pos, ScreenPos};

use super::Player;

pub trait FollowCam {
    fn update(&mut self, player: &Player);
}

impl FollowCam for Camera {
    fn update(&mut self, player: &Player) {
        self.pos.x = player.pos.x - Coord::from(self.width) / 2.0;
        self.pos.y = player.pos.y - Coord::from(self.height) / 2.0;
    }
}
