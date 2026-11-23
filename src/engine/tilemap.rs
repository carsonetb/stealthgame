use std::collections::HashMap;

use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tiles::prelude::*;

use crate::engine::bitmap::{Bitmap, DIRECTIONS, Neighbors};
use crate::engine::load_ron;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileSet {
    pub atlas: String,
    pub width: usize,
    pub height: usize,
    pub tilesize: UVec2,
    pub tiles: HashMap<Neighbors, UVec2>,
    pub default: UVec2,
    pub empty: UVec2,
}

impl TileSet {
    pub fn new(
        texture: &str,
        width: usize,
        height: usize,
        tilesize: UVec2,
        default: UVec2,
        empty: UVec2,
    ) -> Self {
        Self {
            atlas: texture.to_string(),
            width,
            height,
            tilesize,
            default,
            empty,
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
pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Entity>,
    pub queue: Vec<(Entity, usize)>,
    bitmap: Bitmap,
    tileset: TileSet,
}

impl TileMap {
    pub fn new(width: usize, height: usize, tileset: TileSet) -> Self {
        Self {
            width,
            height,
            bitmap: Bitmap::new(width, height, IVec2::ZERO),
            queue: Vec::new(),
            tileset,
            tiles: vec![Entity::new(); width * height],
        }
    }

    pub fn set_tile_bit_basic(&mut self, pos: UVec2, value: bool) {
        self.bitmap.set(pos.as_ivec2(), value);
        let entity = self.tiles[pos.y as usize * self.width + pos.x as usize];
        let index = if value {
            if let Some(&tile) = self
                .tileset
                .tiles
                .get(&self.bitmap.neighbors(pos.as_ivec2()))
            {
                self.tileset.get_tile_index(tile)
            } else {
                self.tileset.get_tile_index(self.tileset.default)
            }
        } else {
            self.tileset.get_tile_index(self.tileset.empty)
        };
        self.queue.push((entity, index));
    }

    pub fn set_tile_bit(&mut self, pos: UVec2, value: bool) {
        self.set_tile_bit_basic(pos, value);

        for direction in DIRECTIONS {
            let pos = pos.as_ivec2() + direction;
            if let Some(bit) = self.bitmap.get(pos) {
                self.set_tile_bit_basic(pos.as_uvec2(), bit);
            }
        }
    }

    pub fn process_system(
        q_tileset: Query<&mut TileMap>,
        mut q_tile: Query<&mut TileTextureIndex>,
    ) {
        for mut tileset in q_tileset {
            for (entity, index) in tileset.queue.drain(..) {
                let mut tile = q_tile.get_mut(entity).unwrap();
                tile.0 = index as u32
            }
        }
    }

    pub fn edit_system(
        mouse_input: Res<ButtonInput<MouseButton>>,
        tilemap_query: Query<
            (
                &mut TileMap,
                &TilemapSize,
                &TilemapGridSize,
                &TilemapTileSize,
                &TilemapType,
                &TilemapAnchor,
                &GlobalTransform,
            ),
            Without<Camera2d>,
        >,
        window_query: Query<&Window>,
        camera_query: Query<(&Camera, &mut GlobalTransform), With<Camera2d>>,
    ) {
        let Ok(window) = window_query.single() else {
            return;
        };
        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };
        let Some(cursor) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        else {
            return;
        };

        for (mut tilemap, map_size, grid_size, tile_size, map_type, anchor, transform) in
            tilemap_query
        {
            if let Some(position) = TilePos::from_world_pos(
                &(cursor / transform.scale().xy()),
                map_size,
                grid_size,
                tile_size,
                map_type,
                anchor,
            ) {
                if mouse_input.pressed(MouseButton::Right) {
                    tilemap.set_tile_bit(position.into(), false);
                }
                if mouse_input.pressed(MouseButton::Left) {
                    tilemap.set_tile_bit(position.into(), true);
                }
            }
        }
    }
}

pub fn create_tilemap_system(
    width: usize,
    height: usize,
    tile_width: u32,
    tile_height: u32,
    tileset: &'static str,
) -> ScheduleConfigs<ScheduleSystem> {
    (move |mut commands: Commands, mut db: ResMut<AtlasDB>, mut assets: ResMut<AssetServer>| {
        db.load(&mut assets, tileset);
        let tileset: TileSet = load_ron(tileset).unwrap();
        let mut tilemap = TileMap::new(width, height, tileset.clone());

        let texture: Handle<Image> = assets.load(tileset.atlas.clone());
        let size = TilemapSize::new(width as u32, height as u32);
        let mut storage = TileStorage::empty(size);

        let entity = commands.spawn_empty().id();
        for x in 0..size.x {
            for y in 0..size.y {
                let position = TilePos::new(x, y);
                let entity = commands
                    .spawn(TileBundle {
                        position,
                        tilemap_id: TilemapId(entity),
                        texture_index: TileTextureIndex(
                            tileset.get_tile_index(tileset.empty) as u32
                        ),
                        ..Default::default()
                    })
                    .id();
                tilemap.tiles[position.y as usize * tilemap.width + position.x as usize] = entity;
                storage.set(&position, entity);
            }
        }

        let tile_size = TilemapTileSize::new(tile_width as f32, tile_height as f32);
        let grid_size = tile_size.into();
        let map_type = TilemapType::default();

        commands.entity(entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size,
            storage,
            texture: TilemapTexture::Single(texture),
            tile_size,
            anchor: TilemapAnchor::Center,
            transform: Transform::from_scale(Vec3::new(2.0, 2.0, 1.0)),
            ..Default::default()
        });
        commands.entity(entity).insert(tilemap);
    })
    .into_configs()
}
