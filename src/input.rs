use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Deref, DerefMut, Default, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerInput(pub Vec2);

pub fn movement(input: Res<ButtonInput<KeyCode>>, mut movement: ResMut<PlayerInput>) {
    **movement = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
}
