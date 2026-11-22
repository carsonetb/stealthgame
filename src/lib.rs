#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use bevy::prelude::*;

pub mod engine;

pub fn init(app: &mut App) {
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        engine::EnginePlugin,
    ));
}
