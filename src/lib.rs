#![allow(clippy::type_complexity)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::too_many_lines)]
// #![allow(clippy::must_use_candidate)]
// #![allow(clippy::needless_pass_by_value)]
// #![allow(clippy::enum_glob_use)]

pub mod ball;
pub mod collider;
pub mod game_logic;
pub mod paddle;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

use crate::{
    ball::BallPlugin, collider::ColliderPlugin, game_logic::GameLogicPlugin,
    paddle::PaddlePlugin
};

pub fn run() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ping Pong".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GameLogicPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ColliderPlugin)
        .add_plugin(PaddlePlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.label("main_setup"))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

fn setup(mut cmd: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(UiCameraBundle::default());

    rapier_config.gravity = Vector2::new(0.0, 0.0);
}
