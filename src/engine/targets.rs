use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalkingState {
    Hold,
    Step,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WalkingTarget {
    pub max_distance: f32,
    pub max_time: f32,
    pub step_factor: f32,
    pub step_tolerance: f32,
    pub pos: Vec2,
    pub start: Vec2,
    pub target: Vec2,
    pub velocity: Vec2,
    pub progress: f32,
    pub state: WalkingState,
    pub step_allowed: bool,
    pub step_height: f32,
    elapsed: Stopwatch,
}

impl WalkingTarget {
    pub fn new(
        max_distance: f32,
        max_time: f32,
        step_factor: f32,
        step_tolerance: f32,
        step_height: f32,
        pos: Vec2,
    ) -> Self {
        Self {
            max_distance,
            max_time,
            step_factor,
            step_tolerance,
            step_height,
            pos,
            start: pos,
            target: pos,
            progress: 0.0,
            state: WalkingState::Hold,
            step_allowed: true,
            velocity: Vec2::ZERO,
            elapsed: Stopwatch::new(),
        }
    }

    pub fn should_step(&self) -> bool {
        self.pos.distance(self.target) > self.max_distance
    }

    pub fn process(&mut self, target: Vec2, dt: Duration) {
        self.target = target;
        self.elapsed.tick(dt);

        match self.state {
            WalkingState::Hold => {
                if (self.pos.distance(self.target) > self.max_distance || false)
                    && self.step_allowed
                {
                    self.state = WalkingState::Step;
                    self.start = self.pos;
                    self.progress = 0.0;
                    self.velocity = Vec2::ZERO;
                }
            }
            WalkingState::Step => {
                let last_pos = self.pos;
                self.progress += self.step_factor * dt.as_secs_f32() * 60.0;
                self.progress = self.progress.min(1.0);
                self.pos = self.start.lerp(self.target, self.progress);
                if self.progress >= 1.0 {
                    self.state = WalkingState::Hold;
                }
                self.pos.y += step_curve(self.step_height, self.progress);
                self.velocity = (self.pos - last_pos) / dt.as_secs_f32();
            }
        }
    }

    pub fn process_system(time: Res<Time>, query: Query<&mut Self>) {
        for mut target in query {
            let target_pos = target.target;
            target.process(target_pos, time.delta());
        }
    }
}

fn step_curve(height: f32, d: f32) -> f32 {
    // https://www.desmos.com/calculator/ugvgeuod7s
    -4.0 * height * (d - 0.5).powi(2) + height
}
