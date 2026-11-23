use bevy::prelude::*;
use stealthgame::{engine::tilemap::create_tilemap_system, init};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    let mut app = App::new();

    init(&mut app);

    app.add_systems(
        Startup,
        (
            setup,
            create_tilemap_system(50, 50, 20, 20, "tileset/example.ron"),
        ),
    );

    app.run();
}
