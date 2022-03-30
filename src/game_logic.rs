use bevy::prelude::*;

use crate::collider::Collider;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    NotReady,
    Ready,
    NewRound,
    Playing,
    Goal,
    // EndRound,
}

#[derive(Default)]
pub struct Score;

#[derive(Component)]
pub struct Despawn;

impl Despawn {
    fn offscreen(
        mut cmd: Commands,
        entities: Query<(Entity, &Transform), With<Despawn>>,
        windows: Res<Windows>,
    ) {
        let window = windows.get_primary().unwrap();

        for (e, transform) in entities.iter() {
            if transform.translation.x.abs() > (window.width() / 2.0)
                || transform.translation.y.abs() > (window.height() / 2.0)
            {
                cmd.entity(e).despawn_recursive();
            }
        }
    }
}

impl Score {
    fn goal(
        mut goal_event: EventReader<GoalEvent>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        for _ in goal_event.iter() {
            game_state.set(GameState::NewRound).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct GoalEvent(pub Collider);

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEvent>()
            .insert_resource(Score::default())
            .add_state(GameState::Ready)
            .add_system(Score::goal)
            .add_system(Despawn::offscreen);
    }
}
