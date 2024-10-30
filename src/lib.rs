use actions::ActionsPlugin;
use bevy::{app::{App, Plugin}, color::Color, prelude::{AppExtStates,States}};
use board::BoardPlugin;
use food::FoodPlugin;
use menu::MenuPlugin;
use snake::SnakePlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod menu;
mod snake;
mod food;
mod board;
mod actions;

pub const ARENA_WIDTH: u32 = 25;
pub const ARENA_HEIGHT: u32 = 25;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.04, 0.04, 0.04);
pub const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
pub const SNAKE_BODY_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);


#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            ActionsPlugin,
            MenuPlugin,
            BoardPlugin,
            SnakePlugin,
            FoodPlugin
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}