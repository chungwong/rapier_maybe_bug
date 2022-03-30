use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ball::{self, Ball};
use crate::game_logic::{GameState, GoalEvent};
use crate::paddle::Side;

const WALL_THICK: f32 = 5.0;
const EDGE_THICK: f32 = 50.0;

const POWER_PUSH_MULTIPLIER: f32 = 2.5;

#[derive(Clone, Component, Copy, Debug)]
pub enum Collider {
    Top,
    Bottom,
    Left,
    Right,
}

impl Collider {
    fn setup_colliders(
        mut cmd: Commands,
        windows: Res<Windows>,
        rapier_config: Res<RapierConfiguration>,
    ) {
        let window = windows.get_primary().unwrap();

        let shape_top_and_bottom_wall = shapes::Rectangle {
            extents: Vec2::new(
                window.width() * rapier_config.scale,
                WALL_THICK * rapier_config.scale,
            ),
            origin: shapes::RectangleOrigin::Center,
        };

        //Spawn top edge
        let top_wall_pos = Vec2::new(0.0, window.height() / 2.);
        cmd.spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_top_and_bottom_wall.extents.x / rapier_config.scale / 2.0,
                shape_top_and_bottom_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            position: top_wall_pos.into(),
            ..Default::default()
        })
        .insert(Collider::Top)
        .insert(Name::new("Top Edge"));

        //Spawn bottom wall
        let bottom_wall_pos = Vec2::new(0.0, -window.height() / 2.);
        cmd.spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_top_and_bottom_wall.extents.x / rapier_config.scale / 2.0,
                shape_top_and_bottom_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            position: bottom_wall_pos.into(),
            ..Default::default()
        })
        .insert(Collider::Bottom)
        .insert(Name::new("Bottom Edge"));

        let shape_left_and_right_wall = shapes::Rectangle {
            extents: Vec2::new(
                EDGE_THICK * rapier_config.scale,
                window.height() * rapier_config.scale,
            ),
            origin: shapes::RectangleOrigin::Center,
        };

        // Offset position to left and right edges.
        let offset = 15.0;
        //Spawn left wall
        let left_wall_pos = Vec2::new(-(window.width() / 2.0 + offset), 0.0);
        cmd.spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_left_and_right_wall.extents.x / rapier_config.scale / 2.0,
                shape_left_and_right_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            collider_type: ColliderType::Sensor.into(),
            position: left_wall_pos.into(),
            ..Default::default()
        })
        .insert(Collider::Left)
        .insert(Name::new("Left Edge"));

        //Spawn right wall
        let right_wall_pos = Vec2::new(window.width() / 2.0 + offset, 0.0);
        cmd.spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_left_and_right_wall.extents.x / rapier_config.scale / 2.0,
                shape_left_and_right_wall.extents.y / rapier_config.scale / 2.0,
            )
            .into(),
            collider_type: ColliderType::Sensor.into(),
            position: right_wall_pos.into(),
            ..Default::default()
        })
        .insert(Collider::Right)
        .insert(Name::new("Right Edge"));
    }

    fn power_push_ball(ball_vel: &mut RigidBodyVelocityComponent, mut speed: f32) {
        speed *= POWER_PUSH_MULTIPLIER;

        ball_vel.linvel.x = (ball_vel.linvel.x * 200.0).clamp(-speed, speed);
        ball_vel.linvel.y = 0.0;
    }

    /// rebounce the ball and regain velocity, and clampped
    // fn bounce_ball(ball_vel: &mut RigidBodyVelocityComponent, speed: f32) {
    //     ball_vel.linvel.x = (ball_vel.linvel.x * 200.0).clamp(-speed, speed);
    //     ball_vel.linvel.y = (ball_vel.linvel.y * 200.0).clamp(-speed, speed);
    // }

    fn contact_events(
        mut key_input: ResMut<Input<KeyCode>>,
        mut contact_events: EventReader<ContactEvent>,
        mut balls: Query<&mut RigidBodyVelocityComponent, With<Ball>>,
        colliders: Query<Entity, With<Collider>>
    ) {
        for contact_event in contact_events.iter() {
            if let ContactEvent::Stopped(h1, h2) = contact_event {
                // check if D or Left key is pressed, we use power_push instead of
                // bounce_ball
                let paddle_push: Option<Side> = if key_input.pressed(KeyCode::D) {
                    Some(Side::Left)
                } else if key_input.pressed(KeyCode::Left) {
                    Some(Side::Right)
                } else {
                    None
                };
                key_input.clear();

                // get collider, is it a paddle or a top/bottom edge?
                let is_edge = colliders
                    .get(h1.entity())
                    .or_else(|_| colliders.get(h2.entity()))
                    .is_ok();

                // Only change linvel of the related ball
                // ball can be h1 or h2, randomly
                // FIXME: tidy it up
                if let Ok(mut ball_vel) = balls.get_mut(h1.entity()) {
                    match (is_edge, paddle_push) {
                        (false, Some(_)) => {
                            Collider::power_push_ball(&mut ball_vel, ball::BALL_SPEED)
                        }
                        // _ => Collider::bounce_ball(&mut ball_vel, ball::BALL_SPEED),
                        _ => {}
                    }
                } else if let Ok(mut ball_vel) = balls.get_mut(h2.entity()) {
                    match (is_edge, paddle_push) {
                        (false, Some(_)) => {
                            Collider::power_push_ball(&mut ball_vel, ball::BALL_SPEED)
                        }
                        // _ => Collider::bounce_ball(&mut ball_vel, ball::BALL_SPEED),
                        _ => {}
                    }
                }
            }
        }
    }

    fn intersection_events(
        mut events: EventReader<IntersectionEvent>,
        mut goal_event: EventWriter<GoalEvent>,
        sides: Query<&Collider>,
        balls: Query<&Transform, With<Ball>>,
    ) {
        for event in events.iter() {
            if let IntersectionEvent {
                intersecting: true,
                collider1: c1,
                collider2: c2,
            } = event
            {
                if let Ok(collider) = sides.get(c1.entity()).or_else(|_| sides.get(c2.entity())) {
                    if balls.get(c1.entity()).or_else(|_| balls.get(c2.entity())).is_ok() {
                        goal_event.send(GoalEvent(*collider));
                    }
                }
            }
        }
    }
}

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Ready).with_system(Collider::setup_colliders),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(Collider::contact_events),
        )
        .add_system(Collider::intersection_events);
    }
}
