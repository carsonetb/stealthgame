use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

pub mod bitmap;
pub mod fabrik;
pub mod targets;
pub mod tilemap;

pub fn load_binary<T: DeserializeOwned>(file: &str) -> Option<T> {
    let bytes =
        std::fs::read(format!("assets/{file}")).expect(&format!("Could not find file {file}"));
    postcard::from_bytes(&bytes).ok()
}

pub fn load_ron<T: DeserializeOwned>(file: &str) -> Option<T> {
    let string = std::fs::read_to_string(format!("assets/{file}"))
        .expect(&format!("Could not find file {file}"));
    ron::from_str(&string).ok()
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
        app.insert_resource(tilemap::AtlasDB::new());

        app.add_systems(Update, targets::WalkingTarget::process_system);
    }
}
