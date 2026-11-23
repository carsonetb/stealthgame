use avian::prelude::*;
use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
};

use serde::{Deserialize, Serialize};

use crate::engine::{CursorPos, GRAVITY, load_ron};

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub struct Player {
    leg_length: f32,
    leg_force: f32,
    leg_damping: f32,
    width: f32,
    height: f32,
}

pub fn player_spawn(player: &'static str) -> ScheduleConfigs<ScheduleSystem> {
    (move |mut commands: Commands, cursor: Res<CursorPos>| {
        let player: Player = load_ron(player).unwrap();

        commands.spawn((
            RigidBody::Dynamic,
            Collider::capsule(player.width, player.height),
            Mass(20.0),
            Transform::from_xyz(cursor.x, cursor.y, 0.0),
            RayCaster::new(Vec2::new(0.0, -player.leg_length), Dir2::NEG_Y).with_max_distance(35.0),
            player,
        ));
    })
    .into_configs()
}

pub fn player_stand(q_player: Query<(&Player, &RayHits, &RigidBody, Forces)>) {
    for (player, hits, body, mut forces) in q_player {
        if let Some(hit) = hits.first() {
            forces.apply_force(Vec2::new(
                0.0,
                player.leg_force * (player.leg_length - hit.distance),
            ));
            forces.apply_force(body.);
        }
    }
}
