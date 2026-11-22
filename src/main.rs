#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use bevy::prelude::*;
use stealthgame::init;

pub mod engine;

fn main() {
    let mut app = App::new();

    init(&mut app);

    app.run();
}
