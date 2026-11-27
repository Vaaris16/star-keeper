use bevy::{log::tracing::Instrument, prelude::*};

use crate::{
    GameState,
    screens::game::{
        game_plugin::GameSet,
        ui::{
            protector_lives::{
                ProtectorLives, enter_retry_screen, protector_lives, reset_protector_lives,
            },
            score::{Score, ScoreTime, reset_score, spawn_score, update_score},
            star_health::{StarHealth, reset_star_health, spawn_health_bar, update_health_bar},
        },
    },
};

pub const TOP_PADDING: i32 = 25;
pub const SIDE_PADDING: i32 = 25;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (protector_lives, spawn_score, spawn_health_bar),
        );

        app.add_systems(Update, update_score.in_set(GameSet))
            .insert_resource(Score::default())
            .insert_resource(ScoreTime::default());

        app.add_systems(Update, enter_retry_screen.in_set(GameSet))
            .insert_resource(ProtectorLives::default());

        app.add_systems(Update, update_health_bar.in_set(GameSet))
            .insert_resource(StarHealth::default());

        app.add_systems(
            OnExit(GameState::Game),
            (reset_protector_lives, reset_star_health),
        )
        .add_systems(OnEnter(GameState::SplashScreen), reset_score);
    }
}
