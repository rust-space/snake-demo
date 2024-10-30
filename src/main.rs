use bevy::prelude::*;
use bevy::window::WindowResolution;
use snake::GamePlugin;
use snake::BACKGROUND_COLOR;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "贪吃蛇demo".to_string(),
                resolution: WindowResolution::new(500.0, 500.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
