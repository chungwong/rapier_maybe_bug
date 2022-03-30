use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_logic::{Despawn, GameState};

const BALL_RADIUS: f32 = 15.;
const BALL_LAUNCH_DELAY: f32 = 1.;
pub const BALL_SPEED: f32 = 500.;

#[derive(Component)]
pub struct Ball {
    linvel: Vec2,
}

impl Ball {
    fn new() -> Self {
        Self {
            linvel: Vec2::new(0.0, 0.0),
        }
    }

    fn setup_ball(
        mut cmd: Commands,
        rapier_config: Res<RapierConfiguration>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        let ball = Ball::new();

        let ball_pos = Vec2::new(0.0, 0.0);

        let shape_ball = shapes::Circle {
            radius: BALL_RADIUS * rapier_config.scale,
            center: Vec2::ZERO,
        };

        cmd.spawn_bundle(GeometryBuilder::build_as(
            &shape_ball,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            }
            .into(),
            damping: RigidBodyDamping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(shape_ball.radius / rapier_config.scale).into(),
            collider_type: ColliderType::Solid.into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            position: ball_pos.into(),
            material: ColliderMaterial {
                restitution: 1.0,
                ..Default::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Despawn)
        .insert(Timer::from_seconds(BALL_LAUNCH_DELAY, false))
        .insert(Name::new("Ball"))
        .insert(ball);

        // change state to playing
        game_state.set(GameState::Playing).unwrap();
    }

    // launch the ball after x second
    fn launch_ball(
        mut cmd: Commands,
        mut balls: Query<(Entity, &mut RigidBodyVelocityComponent, &mut Timer), With<Ball>>,
        time: Res<Time>,
    ) {
        for (entity, mut vel, mut timer) in balls.iter_mut() {
            if timer.tick(time.delta()).just_finished() {
                // vel.linvel = ball.linvel.into();
                vel.linvel = Vec2::new(BALL_SPEED * 2.5, 0.0).into();
                cmd.entity(entity).remove::<Timer>();
            }
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::NewRound).with_system(Ball::setup_ball))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(Ball::launch_ball),
            );
    }
}
