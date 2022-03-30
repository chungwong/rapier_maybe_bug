use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_logic::GameState;

const PADDLE_WIDTH: f32 = 40.;
const PADDLE_HEIGHT: f32 = 100.;
const PADDLE_SPEED: f32 = 400.;

#[derive(Component, Debug, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Ai;

#[derive(Component, Debug)]
pub struct Paddle {
    pub side: Side,
    pub speed: f32,
}

impl Paddle {
    fn left() -> Self {
        Self {
            side: Side::Left,
            speed: PADDLE_SPEED,
        }
    }

    fn right() -> Self {
        Self {
            side: Side::Right,
            speed: PADDLE_SPEED * 1.5,
        }
    }

    fn spawn(cmd: &mut Commands, paddle: Self, translation: Vec3) {
        let position = Vec2::new(translation.x, translation.y);

        cmd.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                translation,
                ..Default::default()
            },
            sprite: Sprite {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicVelocityBased.into(),
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(0.0, 0.0).into(),
                angvel: 0.0,
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0).into(),
            position: position.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Name::new(format!("Paddle {:?}", paddle.side)))
        .insert(paddle);
    }

    fn setup_paddle(
        mut cmd: Commands,
        windows: Res<Windows>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        let window = windows.get_primary().unwrap();

        Paddle::spawn(
            &mut cmd,
            Paddle::left(),
            Vec3::new(-window.width() / 2.0, 1.0, 1.0),
        );
        Paddle::spawn(
            &mut cmd,
            Paddle::right(),
            Vec3::new(window.width() / 2.0, 1.0, 1.0),
        );

        // change state to new round and start game(spawn ball)
        game_state.set(GameState::NewRound).unwrap();
    }

    fn movement(
        key_input: Res<Input<KeyCode>>,
        mut paddles: Query<(
            &mut RigidBodyVelocityComponent,
            &RigidBodyPositionComponent,
            &Paddle,
        )>,
        windows: Res<Windows>,
    ) {
        if key_input.pressed(KeyCode::W)
            || key_input.pressed(KeyCode::S)
            || key_input.pressed(KeyCode::Up)
            || key_input.pressed(KeyCode::Down)
        {
            let window = windows.get_primary().unwrap();
            let max_y = window.height() / 2.0 - PADDLE_HEIGHT / 2.0;

            for (mut rb_vel, rb_pos, paddle) in paddles.iter_mut() {
                let vel_y = paddle.speed;

                let paddle_y = rb_pos.position.translation.y;

                match paddle.side {
                    Side::Left => {
                        if key_input.pressed(KeyCode::W) && paddle_y <= max_y {
                            rb_vel.linvel = Vec2::new(0.0, vel_y).into();
                        } else if key_input.pressed(KeyCode::S) && paddle_y >= -max_y {
                            rb_vel.linvel = Vec2::new(0.0, -vel_y).into();
                        }
                    }
                    Side::Right => {
                        if key_input.pressed(KeyCode::Up) && paddle_y <= max_y {
                            rb_vel.linvel = Vec2::new(0.0, vel_y).into();
                        } else if key_input.pressed(KeyCode::Down) && paddle_y >= -max_y {
                            rb_vel.linvel = Vec2::new(0.0, -vel_y).into();
                        }
                    }
                }
            }
        }
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Ready).with_system(Paddle::setup_paddle))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(Paddle::movement));
    }
}
