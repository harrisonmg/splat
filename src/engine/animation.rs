use std::time::{Duration, Instant};

use super::Sprite;

pub struct Animation {
    frames: Vec<Sprite>,
    current_frame: usize,
    frame_time: Duration,
    last_frame_time: Instant,
    one_shot: bool,
}

impl Animation {
    pub fn new(frames: Vec<Sprite>, frame_time: Duration, one_shot: bool) -> Self {
        Self {
            frames,
            frame_time,
            current_frame: 0,
            last_frame_time: Instant::now(),
            one_shot,
        }
    }

    pub fn get_frame(&mut self) -> &Sprite {
        if self.last_frame_time.elapsed() > self.frame_time {
            if self.current_frame < self.frames.len() - 1 {
                self.current_frame += 1;
                self.last_frame_time = Instant::now();
            } else if !self.one_shot {
                self.current_frame = 0;
                self.last_frame_time = Instant::now();
            }
        }

        &self.frames[self.current_frame]
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = Instant::now();
    }

    pub fn done(&self) -> bool {
        self.one_shot && self.current_frame == self.frames.len() - 1
    }
}
