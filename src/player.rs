use std::f32::consts::PI;

use aseprite::prelude::*;
use avian::prelude::*;
use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
};

use serde::{Deserialize, Serialize};

use crate::{
    engine::{
        CursorPos, GameLayer, load_ron,
        targets::{WalkingState, WalkingTarget},
    },
    input::PlayerInput,
};

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub struct Player {
    spritesheet: String,

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
    hand_force: f32,
    hand_damping: f32,
    arm_max_distance: f32,
    arm_offset: Vec2,

    foot_radius: f32,
    foot_mass: f32,
    leg_offset: Vec2,

    step_max_distance: f32,
    step_max_time: f32,
    step_factor: f32,
    step_tolerance: f32,
    step_height: f32,
}

#[derive(Component, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Hand;

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Leg(Entity);

#[derive(Component, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Foot;

pub fn player_spawn(player: &'static str) -> ScheduleConfigs<ScheduleSystem> {
    (move |mut commands: Commands, cursor: Res<CursorPos>, server: Res<AssetServer>| {
        let player: Player = load_ron(player).unwrap();

        let cursor = *cursor.clone();

        commands
            .spawn((
                RigidBody::Dynamic,
                Collider::capsule(player.width, player.height),
                Mass(player.mass),
                Dominance(1),
                CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Player]),
                Transform::from_xyz(cursor.x, cursor.y, 0.0),
                RayCaster::new(Vec2::new(0.0, -player.height), Dir2::NEG_Y)
                    .with_max_distance(player.leg_length)
                    .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Default)),
                player.clone(),
            ))
            .with_children(|parent| {
                // let player_entity = parent.target_entity();

                parent.spawn((
                    AseAnimation {
                        aseprite: server.load(&player.spritesheet),
                        animation: Animation::tag("stand"),
                    },
                    Sprite::default(),
                    Transform::from_xyz(-3.0, -8.0, 0.0).with_scale(Vec3::splat(3.0)),
                ));

                let left_hand = parent.spawn(hand_bundle(&player, -player.width - 5.0)).id();
                parent.spawn(
                    DistanceJoint::new(parent.target_entity(), left_hand)
                        .with_max_distance(player.arm_max_distance)
                        .with_local_anchor1(player.arm_offset.with_x(-player.arm_offset.x)),
                );

                let right_hand = parent.spawn(hand_bundle(&player, player.width + 5.0)).id();
                parent.spawn(
                    DistanceJoint::new(parent.target_entity(), right_hand)
                        .with_max_distance(player.arm_max_distance)
                        .with_local_anchor1(player.arm_offset),
                );

                //                 let left_leg_start = cursor + 0.0 + Vec2::NEG_Y * player.leg_length;
                //                 let foot = parent.spawn(foot_bundle(&player, left_leg_start)).id();
                //                 parent.spawn(leg_bundle(&player, left_leg_start, player_entity, foot));
                //
                //                 let right_leg_start = cursor - 20.0 + Vec2::NEG_Y * player.leg_length;
                //                 let foot = parent.spawn(foot_bundle(&player, right_leg_start)).id();
                //                 parent.spawn(leg_bundle(&player, right_leg_start, player_entity, foot));
            });
    })
    .into_configs()
}

fn hand_bundle(player: &Player, x_offset: f32) -> impl Bundle {
    (
        Hand,
        RigidBody::Dynamic,
        Collider::circle(player.hand_radius),
        Mass(player.hand_mass),
        Dominance(1),
        CollisionLayers::new(
            GameLayer::PlayerArms,
            [GameLayer::Default, GameLayer::PlayerArms],
        ),
        Transform::from_xyz(x_offset, 0.0, 0.0),
    )
}

fn leg_bundle(player: &Player, start: Vec2, player_entity: Entity, foot: Entity) -> impl Bundle {
    (
        Leg(foot),
        Transform::from_xyz(0.0, 0.0, 0.0),
        WalkingTarget::new(
            player.step_max_distance,
            player.step_max_time,
            player.step_factor,
            player.step_tolerance,
            player.step_height,
            start,
        ),
        RayCaster::new(player.leg_offset, Dir2::NEG_Y)
            .with_max_distance(player.leg_length)
            .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Default)),
        DistanceJoint::new(player_entity, foot)
            .with_local_anchor1(player.leg_offset)
            .with_max_distance(f32::MAX)
            .with_compliance(0.005),
    )
}

fn foot_bundle(player: &Player, start: Vec2) -> impl Bundle {
    (
        Foot,
        RigidBody::Dynamic,
        Collider::circle(player.foot_radius),
        Mass(player.foot_mass),
        Dominance(0),
        Transform::from_xyz(start.x, start.y, 0.0),
        CollisionLayers::new(
            GameLayer::PlayerLegs,
            [GameLayer::Default, GameLayer::Player, GameLayer::PlayerLegs],
        ),
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
    mut q_hands: Query<Forces, (With<Hand>, Without<Player>)>,
) {
    for (player, transform, mut player_forces, children) in q_player {
        for &child in children {
            if let Ok(mut hand_forces) = q_hands.get_mut(child) {
                let force = (**mouse - transform.translation().xy()) * player.hand_force;
                hand_forces.apply_force(force);
                hand_forces.apply_force(-hand_forces.linear_velocity() * player.hand_damping);
                player_forces.apply_force_at_point(
                    -force,
                    transform
                        .transform_point(Vec3::new(0.0, player.arm_offset.y, 0.0))
                        .xy(),
                );
            }
        }
    }
}

pub fn player_feet(
    mut commands: Commands,
    q_player: Query<(&Player, &LinearVelocity, &GlobalTransform, &Children)>,
    mut q_legs: Query<(
        &Leg,
        &RayCaster,
        &RayHits,
        &mut WalkingTarget,
        &mut DistanceJoint,
    )>,
    mut q_feet: Query<(Entity, &RigidBody, &mut Position), (With<Foot>, Without<Player>)>,
) {
    for (player, velocity, transform, children) in q_player {
        let mut any_stepping = false;
        let mut stepping_entity = None;
        for &child in children {
            if let Ok((leg, raycast, hits, mut target, mut joint)) = q_legs.get_mut(child) {
                if let Some(data) = hits.first() {
                    joint.limits.max = f32::MAX;

                    target.max_distance = velocity.x.abs() * 0.15;
                    let right = data.normal.rotate_angle(-velocity.x.signum() * PI / 2.0);
                    target.target = transform
                        .transform_point(
                            (raycast.origin + Vec2::NEG_Y * (data.distance - player.foot_radius))
                                .extend(0.0),
                        )
                        .xy()
                        + right * velocity.x.abs() * 0.15;

                    if let Ok((entity, &typ, mut position)) = q_feet.get_mut(leg.0) {
                        if typ == RigidBody::Dynamic {
                            commands.entity(entity).insert(RigidBody::Kinematic);
                        }
                        **position = target.pos;
                    }

                    if target.state == WalkingState::Step {
                        any_stepping = true;
                        stepping_entity = Some(child);
                    }
                } else {
                    joint.limits.max = player.leg_length;

                    let mut pos = target.pos;
                    if let Ok((entity, &typ, position)) = q_feet.get_mut(leg.0) {
                        if typ == RigidBody::Kinematic {
                            // **velocity = target.velocity / 1.7;
                            commands.entity(entity).insert(RigidBody::Dynamic);
                        }
                        pos = **position;
                    }
                    target.state = WalkingState::Hold;
                    target.step_allowed = false;
                    target.pos = pos;
                }
            }
        }

        if !any_stepping {
            for &child in children {
                if let Ok((_, _, _, target, _)) = q_legs.get(child) {
                    if target.should_step() {
                        any_stepping = true;
                        stepping_entity = Some(child);
                        break;
                    }
                }
            }
        }

        for &child in children {
            if let Ok((_, _, _, mut target, _)) = q_legs.get_mut(child) {
                if Some(child) == stepping_entity {
                    target.step_allowed = true;
                } else {
                    target.step_allowed = !any_stepping;
                }
            }
        }
    }
}
