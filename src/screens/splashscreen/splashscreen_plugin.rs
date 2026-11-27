use bevy::{
    prelude::*,
    text::{FontSourceTemplate, LetterSpacing},
};

use crate::{GameState, core::game_fonts::fonts::GameFonts};

pub struct SplashScreenPlugin;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct SplashScreenSet;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            SplashScreenSet.run_if(in_state(GameState::SplashScreen)),
        );
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_splash_screen)
            .add_systems(Update, check_key_presses.in_set(SplashScreenSet))
            .add_systems(OnExit(GameState::SplashScreen), cleanup_splash_screen);
    }
}

#[derive(Component, FromTemplate)]
struct SplashScreen;

fn spawn_splash_screen(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(25)
        }
        SplashScreen
        Children [
            @spawn_splash_title()
            --
            @spawn_start_text()
        ]
    });
}

fn spawn_splash_title() -> impl Scene {
    let splash_title_font = GameFonts::MontserratMedium.path();
    bsn! {
        Text("STARKEEPER")
        TextFont {
            font: FontSourceTemplate::Handle(splash_title_font),
            font_size: px(100)
        }
        TextColor(Color::WHITE)
        LetterSpacing::Px(17.5)
    }
}

fn spawn_start_text() -> impl Scene {
    let splash_title_font = GameFonts::MontserratMedium.path();
    bsn! {
        Text("PRESS ANY KEY")
        TextFont {
            font: FontSourceTemplate::Handle(splash_title_font),
            font_size: px(25)
        }
        TextColor(Color::hsl(255., 0.36, 0.72))
        LetterSpacing::Px(17.5)
    }
}

fn check_key_presses(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.get_just_pressed().next().is_some() {
        next_state.set(GameState::Game);
    }
}

fn cleanup_splash_screen(
    splash_screen: Single<Entity, With<SplashScreen>>,
    mut commands: Commands,
) {
    commands.entity(*splash_screen).despawn();
}
