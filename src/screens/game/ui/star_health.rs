use bevy::prelude::*;

use crate::screens::game::ui::game_ui::TOP_PADDING;

#[derive(Component, FromTemplate)]
pub struct StarHealthBar;

#[derive(Resource)]
pub struct StarHealth(pub f32);

impl Default for StarHealth {
    fn default() -> Self {
        Self(100.)
    }
}

pub fn spawn_health_bar(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            position_type: PositionType::Absolute,
            top: px(TOP_PADDING),
            justify_content: JustifyContent::Center,
        }
        Children[
            Node {
                width: px(250.0),
                height: px(12.0),
            }
            BackgroundColor(Color::hsl(0., 0., 0.25))
            Children[
                Node {
                    width: percent(100),
                    height: percent(100),
                }
                BackgroundColor(Color::hsl(45., 0.75, 0.60))
                StarHealthBar
            ]
        ]
    });
}

pub fn update_health_bar(
    star_health: Res<StarHealth>,
    mut star_health_bar: Single<&mut Node, With<StarHealthBar>>,
) {
    let health = star_health.0;
    let max_health = StarHealth::default().0;

    let percentage = (health / max_health) * 100.;
    star_health_bar.width = percent(percentage as f32);
}

pub fn reset_star_health(mut health: ResMut<StarHealth>) {
    health.0 = StarHealth::default().0;
}
