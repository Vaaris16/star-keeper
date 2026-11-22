use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct BgImage;

pub fn set_bg_image(mut commands: Commands, window: Single<&Window>) {
    commands.spawn_scene(bsn! {
        Sprite {
            image: "bg_image.png",
            custom_size: Vec2::new(window.width(), window.height())
        }
        BgImage
    });
}

pub fn update_bg_image_size(
    mut bg_image: Single<&mut Sprite, With<BgImage>>,
    window: Single<&Window>,
) {
    bg_image.custom_size = Some(Vec2::new(window.width(), window.height()));
}
