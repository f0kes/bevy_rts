use std::time::Duration;

use bevy::math::curve::{EaseFunction, EasingCurve};
use bevy::prelude::*;

pub struct TweenData {
    pub ease_function: EaseFunction,
    pub duration: Duration,
    pub offset: f32,
    pub repeat: RepeatCount,
    pub repeat_strategy: RepeatStrategy,
    pub time: f32,
}
impl Default for TweenData {
    fn default() -> Self {
        Self {
            ease_function: EaseFunction::Linear,
            duration: Duration::from_secs_f32(1.0),
            offset: 0.0,
            repeat: RepeatCount::Count(1),
            repeat_strategy: RepeatStrategy::Restart,
            time: 0.0,
        }
    }
}
impl TweenData {
    pub fn sample(&self) -> f32 {
        let time = self.time;
        let cycle_duration = self.duration.as_secs_f32();
        let mut normalized_time = (time / cycle_duration) - self.offset;

        normalized_time = match self.repeat_strategy {
            RepeatStrategy::Restart => normalized_time % 1.0,
            RepeatStrategy::Mirror => normalized_time % 1.0,
            RepeatStrategy::Reverse => {
                let cycle = normalized_time.floor();
                let t = normalized_time % 1.0;
                if cycle as i32 % 2 == 0 {
                    t
                } else {
                    1.0 - t
                }
            }
        };
        match self.repeat {
            RepeatCount::Infinite => {
                normalized_time = normalized_time % 1.0;
            }
            RepeatCount::Count(count) => {
                if normalized_time >= count as f32 {
                    return match self.repeat_strategy {
                        RepeatStrategy::Restart => 0.0,
                        RepeatStrategy::Reverse => 1.0,
                        RepeatStrategy::Mirror => 1.0,
                    };
                }
                normalized_time = normalized_time % 1.0;
            }
        }
        let (start, end) = match self.repeat_strategy {
            RepeatStrategy::Mirror => (0.0, 1.0),
            _ => (0.0, 1.0),
        };
        EasingCurve::<f32>::new(start, end, self.ease_function)
            .sample(normalized_time)
            .unwrap()
    }
    pub fn with_random_time(mut self) -> Self {
        self.time = rand::random::<f32>() * self.duration.as_secs_f32();
        self
    }
}

pub enum RepeatCount {
    Infinite,
    Count(usize),
}
pub enum RepeatStrategy {
    Restart,
    Reverse,
    Mirror,
}
