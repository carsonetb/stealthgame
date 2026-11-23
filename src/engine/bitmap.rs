use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
];

#[derive(Debug, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Neighbors(pub [bool; 8]);

impl Neighbors {
    pub fn new() -> Self {
        Self([false, false, false, false, false, false, false, false])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bitmap {
    pub values: Vec<bool>,
    pub width: usize,
    pub height: usize,
    pub offset: IVec2,
}

impl Bitmap {
    pub fn new(width: usize, height: usize, offset: IVec2) -> Self {
        Self {
            values: vec![false; width * height],
            width,
            height,
            offset,
        }
    }

    fn to_index(&self, pos: IVec2) -> usize {
        let pos = self.local(pos);
        pos.y as usize * self.width + pos.x as usize
    }

    fn local(&self, pos: IVec2) -> IVec2 {
        pos - self.offset
    }

    pub fn validate(&self, pos: IVec2) -> bool {
        let pos = self.local(pos);
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            false
        } else {
            true
        }
    }

    pub fn set(&mut self, pos: IVec2, value: bool) {
        if !self.validate(pos) {
            panic!("Attempted to retrieve tile in bitmap that was out of bounds! Position: {pos}");
        }
        let index = self.to_index(pos);
        self.values[index] = value;
    }

    pub fn get(&self, pos: IVec2) -> Option<bool> {
        if !self.validate(pos) {
            None
        } else {
            Some(self.values[self.to_index(pos)])
        }
    }

    pub fn neighbors(&self, pos: IVec2) -> Neighbors {
        let mut out = Neighbors::new();
        for (i, &dir) in DIRECTIONS.iter().enumerate() {
            out[i] = if let Some(on) = self.get(pos + dir) {
                on
            } else {
                false
            };
        }
        out[0] = out[0] && out[1] && out[3];
        out[2] = out[2] && out[1] && out[4];
        out[5] = out[5] && out[3] && out[6];
        out[7] = out[7] && out[6] && out[4];
        out
    }
}
