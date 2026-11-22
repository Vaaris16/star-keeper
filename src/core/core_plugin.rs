use bevy::prelude::*;

use crate::core::{
    background::background_image::{set_bg_image, update_bg_image_size},
    player::spawn_player::spawn_player,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_bg_image)
            .add_systems(Update, update_bg_image_size)
            .add_systems(Startup, spawn_player);
    }
}
