use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Fabrik {
    pub points: Vec<Vec2>,
    pub lengths: Vec<f32>,
    pub length: f32,
}

impl Fabrik {
    pub fn new(points: Vec<Vec2>) -> Self {
        if points.len() < 2 {
            panic!("Inverse kinematics requires at least two points!")
        }

        let mut lengths = Vec::with_capacity(points.len() - 1);
        for (i, &point) in points.iter().enumerate().skip(1) {
            lengths.push(point.distance(points[i - 1]));
        }
        Self {
            points,
            length: lengths.iter().sum(),
            lengths,
        }
    }

    pub fn solve(&mut self, target: Vec2, tolerance: f32) -> i32 {
        let target = if !self.solvable(target) {
            self.points[0] + (target - self.points[0]).clamp_length_max(self.length)
        } else {
            target
        };

        let mut iterations = 0;

        while self.iteration(target) > tolerance {
            iterations += 1;
        }

        iterations
    }

    pub fn solvable(&self, target: Vec2) -> bool {
        self.points[0].distance(target) <= self.length
    }

    pub fn iteration(&mut self, target: Vec2) -> f32 {
        let start = self.points[0];

        *self.points.last_mut().unwrap() = target;
        for i in (0..self.points.len() - 1).rev() {
            let next = self.points[i + 1];
            let current = self.points[i];
            let len_share = self.lengths[i] / (next - current).length();
            self.points[i] = (1.0 - len_share) * next + len_share * current;
        }

        self.points[0] = start;
        for i in 0..self.points.len() - 1 {
            let next = self.points[i + 1];
            let current = self.points[i];
            let len_share = self.lengths[i] / (next - current).length();
            self.points[i + 1] = (1.0 - len_share) * current + len_share * next;
        }

        self.points.last().unwrap().distance(target)
    }
}
