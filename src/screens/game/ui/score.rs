use bevy::{prelude::*, text::FontSourceTemplate};

use crate::{
    GameState,
    core::game_fonts::fonts::GameFonts,
    screens::game::{
        game_plugin::GameSet,
        ui::game_ui::{SIDE_PADDING, TOP_PADDING},
    },
};

#[derive(Resource, Default)]
pub struct Score(pub i64);

#[derive(Component, FromTemplate)]
pub struct ScoreText;

#[derive(Resource)]
pub struct ScoreTime(pub Timer);

impl Default for ScoreTime {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

pub fn spawn_score(mut commands: Commands) {
    let score_text_font = GameFonts::MontserratMedium.path();

    commands.spawn_scene(bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: px(TOP_PADDING),
            left: px(SIDE_PADDING),
        }
        Children[
            Text("0")
            TextFont {
                font: FontSourceTemplate::Handle(score_text_font),
                font_size: px(50),
            }
            TextColor(Color::WHITE)
            ScoreText
        ]
    });
}

pub fn update_score(
    time: Res<Time>,
    mut score_time: ResMut<ScoreTime>,
    mut score: ResMut<Score>,
    mut score_text: Single<&mut Text, With<ScoreText>>,
) {
    score_time.0.tick(time.delta());

    if score_time.0.just_finished() {
        score.0 += 1;
        score_text.0 = format!("{}", score.0);
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = Score::default().0;
}
