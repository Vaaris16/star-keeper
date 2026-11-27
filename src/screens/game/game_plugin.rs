use bevy::prelude::*;

use crate::{GameState, screens::game::ui::game_ui::GameUiPlugin};

pub struct GamePlugin;

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub struct GameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), spawn_star)
            .add_plugins(GameUiPlugin);
    }
}

#[derive(Component, FromTemplate)]
struct Star;

fn spawn_star(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Sprite {
            image: "star.png",
            custom_size: Vec2::splat(250.),
        }
        Transform::from_xyz(0.,0.,1.)
        Star
    });
}
