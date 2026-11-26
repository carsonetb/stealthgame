use avian::prelude::*;
use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

pub mod bitmap;
pub mod fabrik;
pub mod targets;
pub mod tilemap;

pub const GRAVITY: f32 = 700.0;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    PlayerArms,
    PlayerLegs,
}

pub fn load_binary<T: DeserializeOwned>(file: &str) -> Option<T> {
    let bytes =
        std::fs::read(format!("assets/{file}")).expect(&format!("Could not find file {file}"));
    postcard::from_bytes(&bytes).ok()
}

pub fn load_ron<T: DeserializeOwned>(file: &str) -> Option<T> {
    let string = std::fs::read_to_string(format!("assets/{file}"))
        .expect(&format!("Could not find file {file}"));
    match ron::from_str(&string) {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Error loading {file}: {e}");
            None
        }
    }
}

pub fn store_binary<T: Serialize>(file: &str, value: T) {
    let bytes = postcard::to_stdvec(&value).expect("Could not serialize struct?");
    std::fs::write(format!("assets/{file}"), bytes).unwrap();
}

pub fn store_ron<T: Serialize>(file: &str, value: T) {
    let string = ron::to_string(&value).expect("Could not serialize struct?");
    std::fs::write(format!("assets/{file}"), string).unwrap();
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>();
        app.insert_resource(tilemap::AtlasDB::new());

        app.add_systems(PreUpdate, cursor_system);
        app.add_systems(
            Update,
            (
                targets::WalkingTarget::process_system,
                tilemap::TileMap::process_system,
                tilemap::TileMap::edit_system,
            ),
        );
    }
}

#[derive(Resource, Deref, DerefMut, Default, Debug, Clone, Copy)]
pub struct CursorPos(pub Vec2);

fn cursor_system(
    mut pos: ResMut<CursorPos>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let Some(cursor) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    else {
        return;
    };
    *pos = CursorPos(cursor);
}
