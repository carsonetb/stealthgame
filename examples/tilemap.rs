use avian::prelude::*;
use bevy::prelude::*;

use stealthgame::{engine::tilemap::create_tilemap_system, init};

fn main() {
    let mut app = App::new();
    app.add_plugins(PhysicsDebugPlugin);

    init(&mut app);

    app.add_systems(
        Startup,
        create_tilemap_system(50, 50, 20, 20, "tileset/example.ron", true),
    );

    app.run();
}
