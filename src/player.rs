use std::f32::consts::PI;

use avian::prelude::*;
use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
};

use serde::{Deserialize, Serialize};

use crate::{
    engine::{CursorPos, GameLayer, load_ron},
    input::PlayerInput,
};

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub struct Player {
    width: f32,
    height: f32,
    mass: f32,

    leg_length: f32,
    leg_force: f32,
    leg_damping: f32,
    walk_force: f32,
    max_leg_rotation: f32,

    angular_force: f32,
    angular_damping: f32,

    hand_radius: f32,
    hand_mass: f32,
    arm_max_distance: f32,
    hand_force: f32,
    hand_damping: f32,

    foot_radius: f32,
}

#[derive(Component, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Limb;

#[derive(Component, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Hand;

pub fn player_spawn(player: &'static str) -> ScheduleConfigs<ScheduleSystem> {
    (move |mut commands: Commands, cursor: Res<CursorPos>| {
        let player: Player = load_ron(player).unwrap();

        let cursor = *cursor.clone();

        commands
            .spawn((
                RigidBody::Dynamic,
                Collider::capsule(player.width, player.height),
                Mass(player.mass),
                CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Player]),
                Transform::from_xyz(cursor.x, cursor.y, 0.0),
                RayCaster::new(Vec2::new(0.0, -player.height), Dir2::NEG_Y)
                    .with_max_distance(player.leg_length)
                    .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Default)),
                player.clone(),
            ))
            .with_children(|parent| {
                let left_hand = parent.spawn(hand_bundle(&player, -20.0)).id();
                parent.spawn(
                    DistanceJoint::new(parent.target_entity(), left_hand)
                        .with_max_distance(player.arm_max_distance)
                        .with_local_anchor1(Vec2::new(-6.0, 6.0)),
                );

                let right_hand = parent.spawn(hand_bundle(&player, 20.0)).id();
                parent.spawn(
                    DistanceJoint::new(parent.target_entity(), right_hand)
                        .with_max_distance(player.arm_max_distance)
                        .with_local_anchor1(Vec2::new(6.0, 6.0)),
                );
            });
    })
    .into_configs()
}

fn hand_bundle(player: &Player, x_offset: f32) -> impl Bundle {
    (
        Limb,
        Hand,
        RigidBody::Dynamic,
        Collider::circle(player.hand_radius),
        Mass(player.hand_mass),
        CollisionLayers::new(
            GameLayer::PlayerArms,
            [GameLayer::Default, GameLayer::PlayerArms],
        ),
        Transform::from_xyz(x_offset, 0.0, 0.0),
    )
}

pub fn player_angle_legs(q_player: Query<(&Player, &Transform, &mut RayCaster)>) {
    for (player, transform, mut raycast) in q_player {
        let max = player.max_leg_rotation.to_radians();
        let target = transform
            .down()
            .truncate()
            .angle_to(Vec2::NEG_Y)
            .clamp(-max, max);
        raycast.direction = Dir2::from_angle(target - PI / 2.0);
    }
}

pub fn player_move(
    input: Res<PlayerInput>,
    q_player: Query<(&Player, &GlobalTransform, &RayHits, Forces)>,
) {
    for (player, transform, hits, mut forces) in q_player {
        if let Some(hit) = hits.first() {
            let point = transform.transform_point(Vec3::new(0.0, 0.8, 0.0)).xy();
            forces.apply_force_at_point(
                **input * player.walk_force * ((player.leg_length - hit.distance) / 20.0).max(1.0),
                point,
            );
        }
    }
}

pub fn player_stand(q_player: Query<(&Player, &RayHits, &Rotation, Forces)>) {
    for (player, hits, rotation, mut forces) in q_player {
        if let Some(hit) = hits.first() {
            forces.apply_force(Vec2::new(
                0.0,
                player.leg_force * (player.leg_length - hit.distance),
            ));
            forces.apply_force(-forces.linear_velocity() * player.leg_damping);

            forces.apply_torque(
                rotation.angle_between(Rotation::from(0.0)).to_degrees()
                    * player.angular_force
                    * (player.leg_length - hit.distance),
            );
            forces.apply_torque(-forces.angular_velocity() * player.angular_damping);
        }
    }
}

pub fn player_hands(
    mouse: Res<CursorPos>,
    q_player: Query<(&Player, &GlobalTransform, Forces, &Children)>,
    mut q_hands: Query<Forces, (With<Hand>, With<Limb>, Without<Player>)>,
) {
    for (player, transform, mut player_forces, children) in q_player {
        for &child in children {
            if let Ok(mut hand_forces) = q_hands.get_mut(child) {
                let force = (**mouse - transform.translation().xy()) * player.hand_force;
                hand_forces.apply_force(force);
                hand_forces.apply_force(-hand_forces.linear_velocity() * player.hand_damping);
                player_forces.apply_force_at_point(
                    -force,
                    transform.transform_point(Vec3::new(0.0, 6.0, 0.0)).xy(),
                );
            }
        }
    }
}
