#![allow(unused)]

#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use avian::prelude::*;
use bevy::input::common_conditions::*;
use bevy::prelude::*;

use stealthgame::engine::GRAVITY;
use stealthgame::init;

use stealthgame::engine::tilemap::create_tilemap_system;
use stealthgame::player::{player_spawn, player_stand};

fn main() {
    let mut app = App::new();
    app.add_plugins(PhysicsDebugPlugin);

    init(&mut app);

    app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY));
    app.add_systems(
        Startup,
        create_tilemap_system(50, 50, 20, 20, "tileset/example.ron", true),
    );
    app.add_systems(
        Update,
        (
            player_spawn("player.ron").run_if(input_just_pressed(MouseButton::Middle)),
            player_stand,
        ),
    );

    app.run();
}
