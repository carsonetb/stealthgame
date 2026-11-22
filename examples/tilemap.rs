use bevy::prelude::*;
use stealthgame::{
    engine::tilemap::{AtlasDB, TileMap},
    init,
};

fn setup(
    mut commands: Commands,
    mut db: ResMut<AtlasDB>,
    mut server: ResMut<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let walls = db.load(&mut server, "resources/walls.ron");
    TileMap::new(walls, UVec2::new(50, 50), Vec2::splat(3.0), 0.0, false).spawn(
        &mut commands,
        &mut db,
        &mut layouts,
    );
}

fn main() {
    let mut app = App::new();

    init(&mut app);

    app.add_systems(Startup, setup);

    app.run();
}
