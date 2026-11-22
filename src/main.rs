use bevy::prelude::*;

use crate::{core::core_plugin::CorePlugin, screens::screens_plugin::ScreensPlugin};

mod core;
mod screens;

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
enum GameState {
    #[default]
    SplashScreen,
    Game,
    Retry,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CorePlugin, ScreensPlugin))
        .init_state::<GameState>()
        .run();
}
