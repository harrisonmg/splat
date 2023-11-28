use std::time::Duration;

pub const UPDATE_RATE: f32 = 100.0; // Hz
pub const DELTA_TIME: f32 = 1.0 / UPDATE_RATE;
pub const UPDATE_INTERVAL: Duration = Duration::from_nanos((DELTA_TIME * 1e9) as u64);
