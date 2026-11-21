#![allow(unused)]

use bevy::prelude::*;

use crate::engine::tilemap::AtlasDB;

mod engine;

fn main() {
    let mut app = App::new();

    app.insert_resource(AtlasDB::new());
}
