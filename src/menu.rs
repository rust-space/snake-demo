use crate::{food::Food, snake::SnakeBlock, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_gameover_menu)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), cleanup_gameover_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button_colors.normal.into(),
                        ..default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::linear_rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct GameOverMenu;

// 游戏结束
pub fn setup_gameover_menu(
    mut cmds: Commands,
    food: Query<Entity, With<Food>>,
    blocks: Query<Entity, With<SnakeBlock>>,
) {
    // 显示 Game Over 弹窗和 Restart 按钮
    cmds.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: bevy::ui::AlignItems::Center,
                justify_content: bevy::ui::JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        GameOverMenu,
    ))
    .with_children(|parent| {
        // 垂直布局，用于显示 "Game Over" 和按钮
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: bevy::ui::FlexDirection::Column,
                    align_items: bevy::ui::AlignItems::Center,
                    margin: bevy::ui::UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                let button_colors = ButtonColors::default();

                // Game Over 文本
                parent.spawn(TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::srgb(1.0, 0.0, 0.0),
                        ..default() // 红色
                    },
                ));
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                margin: bevy::ui::UiRect::all(Val::Px(10.0)),
                                justify_content: bevy::ui::JustifyContent::Center,
                                align_items: bevy::ui::AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.5, 0.5, 0.5).into(),
                            ..default()
                        },
                        button_colors,
                        ChangeState(GameState::Playing),
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "Restart",
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE, // 白色
                                ..default()
                            },
                        ));
                    });
            });
    });

    for entity in food.iter().chain(blocks.iter()) {
        cmds.entity(entity).despawn();
    }
}

fn cleanup_gameover_menu(mut commands: Commands, menu: Query<Entity, With<GameOverMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
