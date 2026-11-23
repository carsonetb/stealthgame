#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use avian::prelude::*;
use bevy::prelude::*;
use tiles::prelude::*;

use crate::engine::EnginePlugin;

pub mod engine;
pub mod player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn init(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        TilemapPlugin,
        PhysicsPlugins::default(),
        EnginePlugin,
    ));
}
