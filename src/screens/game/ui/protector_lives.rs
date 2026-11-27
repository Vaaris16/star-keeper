use crate::{
    GameState,
    screens::game::ui::game_ui::{SIDE_PADDING, TOP_PADDING},
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct ProtectorLives(pub i32);

#[derive(Component, FromTemplate)]
pub struct ProtectorLifeUi;

#[derive(Component, FromTemplate)]
pub struct ProtectorLivesIndicator {
    index: i32,
}

impl Default for ProtectorLives {
    fn default() -> Self {
        Self(3)
    }
}

pub fn protector_lives(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        ProtectorLifeUi
        Node {
            position_type: PositionType::Absolute,
            top: px(TOP_PADDING),
            right: px(SIDE_PADDING),
            flex_direction: FlexDirection::Row,
            column_gap: px(30),
            justify_content: JustifyContent::FlexEnd,
        }
        Children [
            ImageNode {
                image: "protector_lives.png"
            }
            ProtectorLivesIndicator { index: 1 }
            Node {
                width: px(65),
                height: px(65),
            }
            --
            ImageNode {
                image: "protector_lives.png"
            }
            ProtectorLivesIndicator { index: 2 }
            Node {
                width: px(65),
                height: px(65),
            }
            --
            ImageNode {
                image: "protector_lives.png"
            }
            ProtectorLivesIndicator { index: 3 }
            Node {
                width: px(65),
                height: px(65),
            }
        ]
    });
}

pub fn decrement_lives(
    mut commands: Commands,
    mut lives: ResMut<ProtectorLives>,
    mut game_state: ResMut<NextState<GameState>>,
    life_indicator: Query<(Entity, &ProtectorLivesIndicator)>,
) {
    if lives.0 <= 0 {
        return;
    }

    lives.0 -= 1;

    for (indicator_entity, indicator) in &life_indicator {
        if indicator.index > lives.0 {
            commands.entity(indicator_entity).despawn();
        }
    }

    if lives.0 == 0 {
        game_state.set(GameState::Retry);
    }
}

pub fn enter_retry_screen(
    mut next_state: ResMut<NextState<GameState>>,
    protector_lives: Res<ProtectorLives>,
) {
    if protector_lives.0 == 0 {
        next_state.set(GameState::Retry);
    }
}

pub fn reset_protector_lives(mut lives: ResMut<ProtectorLives>) {
    lives.0 = ProtectorLives::default().0;
}
