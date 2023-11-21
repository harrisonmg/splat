use std::time::Duration;

use crate::game::UPDATE_INTERVAL;

use super::Sprite;

pub struct Animation {
    frames: Vec<Sprite>,
    current_frame: usize,
    frame_time: Duration,
    frame_time_left: Duration,
    one_shot: bool,
}

impl Animation {
    pub fn new(frames: Vec<Sprite>, frame_time: Duration, one_shot: bool) -> Self {
        Self {
            frames,
            current_frame: 0,
            frame_time,
            frame_time_left: frame_time,
            one_shot,
        }
    }

    pub fn update(&mut self) {
        match self.frame_time_left.checked_sub(UPDATE_INTERVAL) {
            Some(time) => self.frame_time_left = time,
            None => {
                self.frame_time_left = self.frame_time;
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else if !self.one_shot {
                    self.current_frame = 0;
                }
            }
        }
    }

    pub fn get_frame(&self) -> &Sprite {
        &self.frames[self.current_frame]
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_time_left = self.frame_time;
    }

    pub fn done(&self) -> bool {
        self.one_shot && self.current_frame == self.frames.len() - 1
    }
}
