use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[repr(usize)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TileNeighbor {
    TopLeft = 0,
    Top = 1,
    TopRight = 2,
    Left = 3,
    Right = 4,
    BottomLeft = 5,
    Bottom = 6,
    BottomRight = 7,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Neighbors {
    neighbors: [bool; 8],
}

impl Neighbors {
    pub fn new() -> Self {
        Self {
            neighbors: [false, false, false, false, false, false, false, false],
        }
    }

    pub fn get(&self, neighbor: TileNeighbor) -> bool {
        self.neighbors[neighbor as usize]
    }

    pub fn set(&mut self, neighbor: TileNeighbor, value: bool) {
        self.neighbors[neighbor as usize] = value;
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
        self.values.insert(self.to_index(pos), value);
    }

    pub fn get(&self, pos: IVec2) -> Option<bool> {
        if !self.validate(pos) {
            None
        } else {
            Some(self.values[self.to_index(pos)])
        }
    }

    pub fn neighbors(&self, pos: IVec2) -> Neighbors {
        const DIRECTIONS: [IVec2; 8] = [
            IVec2::new(-1, 1),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
        ];

        let mut out = Neighbors::new();
        for (i, &dir) in DIRECTIONS.iter().enumerate() {
            out.neighbors[i] = if let Some(on) = self.get(pos + dir) {
                on
            } else {
                false
            };
        }
        out
    }
}
