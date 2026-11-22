use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::engine::bitmap::Neighbors;
use crate::engine::load_ron;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileSet {
    pub atlas: String,
    pub width: usize,
    pub height: usize,
    pub tilesize: UVec2,
    pub tiles: HashMap<Neighbors, UVec2>,
    pub default: UVec2,
}

impl TileSet {
    pub fn new(
        texture: &str,
        width: usize,
        height: usize,
        tilesize: UVec2,
        default: UVec2,
    ) -> Self {
        Self {
            atlas: texture.to_string(),
            width,
            height,
            tilesize,
            default,
            tiles: HashMap::new(),
        }
    }

    pub fn get_atlas_pos(&self, bitmask: Neighbors) -> Option<UVec2> {
        Some(self.tiles.get(&bitmask)? * self.tilesize)
    }

    pub fn get_tile_pos(&self, bitmask: Neighbors) -> Option<UVec2> {
        self.tiles.get(&bitmask).cloned()
    }

    pub fn get_tile_index(&self, tile: UVec2) -> usize {
        tile.y as usize * self.width + tile.x as usize
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

    pub fn load(&mut self, server: &mut ResMut<AssetServer>, resource: &str) -> usize {
        let tileset: TileSet = load_ron(resource).unwrap();
        let atlas = server.load(&tileset.atlas);
        self.atlases.push((tileset, atlas));
        self.atlases.len() - 1
    }
}

#[derive(Component, Debug, Clone)]
pub struct TileMapTile {
    pub pos: UVec2,
    pub tilemap: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct TileMap {
    atlas: usize,
    pub scale: Vec2,
    pub size: UVec2,
    pub tiles: Vec<Entity>,
    pub autotile: bool,
    z: f32,
}

impl TileMap {
    pub fn new(atlas: usize, size: UVec2, scale: Vec2, z: f32, autotile: bool) -> Self {
        Self {
            atlas,
            scale,
            size,
            autotile,
            tiles: Vec::new(),
            z,
        }
    }

    pub fn spawn(
        mut self,
        commands: &mut Commands,
        db: &mut ResMut<AtlasDB>,
        layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let tilemap = commands.spawn_empty().id();
        let (tileset, atlas) = &db.atlases[self.atlas];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let layout = TextureAtlasLayout::from_grid(
                    tileset.tilesize,
                    tileset.width as u32,
                    tileset.height as u32,
                    None,
                    None,
                );
                let layout = layouts.add(layout);
                self.tiles.push(
                    commands
                        .spawn((
                            TileMapTile {
                                pos: UVec2::new(x, y),
                                tilemap,
                            },
                            Sprite::from_atlas_image(
                                atlas.clone(),
                                TextureAtlas {
                                    layout,
                                    index: tileset.get_tile_index(tileset.default),
                                },
                            ),
                            Transform::default()
                                .with_translation(Vec3::new(
                                    (x * tileset.tilesize.x) as f32 * self.scale.x,
                                    (y * tileset.tilesize.y) as f32 * self.scale.y,
                                    self.z,
                                ))
                                .with_scale(Vec3::new(self.scale.x, self.scale.y, 1.0)),
                        ))
                        .id(),
                );
            }
        }
    }
}
