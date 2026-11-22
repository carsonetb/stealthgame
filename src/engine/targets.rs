use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    pub target: Vec2,
    pub state: WalkingState,
    pub step_allowed: bool,
    elapsed: Stopwatch,
}

impl WalkingTarget {
    pub fn new(
        max_distance: f32,
        max_time: f32,
        step_factor: f32,
        step_tolerance: f32,
        pos: Vec2,
    ) -> Self {
        Self {
            max_distance,
            max_time,
            step_factor,
            step_tolerance,
            pos,
            target: pos,
            state: WalkingState::Hold,
            step_allowed: true,
            elapsed: Stopwatch::new(),
        }
    }

    pub fn process(&mut self, target: Vec2, dt: Duration) {
        self.target = target;
        self.elapsed.tick(dt);

        match self.state {
            WalkingState::Hold => {
                if (self.pos.distance(self.target) > self.max_distance
                    || self.elapsed.elapsed_secs() > self.max_time)
                    && self.step_allowed
                {
                    self.state = WalkingState::Step;
                }
            }
            WalkingState::Step => {
                self.pos = self.pos.lerp(
                    self.target,
                    1.0 - (-self.step_factor * dt.as_secs_f32()).exp(),
                );
                if self.pos.distance(self.target) < self.step_tolerance {
                    self.state = WalkingState::Hold;
                }
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
