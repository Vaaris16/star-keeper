use bevy::prelude::*;

use crate::screens::{
    game::game_plugin::GamePlugin, splashscreen::splashscreen_plugin::SplashScreenPlugin,
};

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SplashScreenPlugin, GamePlugin));
    }
}
