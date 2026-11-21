use std::collections::HashSet;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::engine::{bitmap::Neighbors, load_ron};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileSetTile {
    pub id: i32,
    pub bitmask: Neighbors,
}

impl TileSetTile {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            bitmask: Neighbors::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileSet {
    pub atlas: String,
    pub width: usize,
    pub height: usize,
    pub tilesize: Vec2,
    pub tiles: Vec<TileSetTile>,
}

impl TileSet {
    pub fn new(texture: &str, width: usize, height: usize, tilesize: Vec2) -> Self {
        Self {
            atlas: texture.to_string(),
            width,
            height,
            tilesize,
            tiles: vec![TileSetTile::new(-1); width * height],
        }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AtlasDB {
    pub atlases: Vec<(TileSet, Handle<Image>)>,
}

impl AtlasDB {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, server: &mut AssetServer, resource: &str) {
        let tileset: TileSet = load_ron(resource).unwrap();
        let atlas = server.load(&tileset.atlas);
        self.atlases.push((tileset, atlas));
    }
}
