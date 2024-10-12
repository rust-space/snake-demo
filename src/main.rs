use bevy::prelude::*;
use bevy::window::WindowResolution;
use snake::events::{GameOverEvent, GrowthEvent}; 
use snake::resources::{FoodSpawnTimer, LastBlockPosition, SnakeBlocks, SnakeMoveTimer};
use snake::systems::{game_over, position_translation, setup_camera, size_scaling, snake_eating, snake_growth, snake_movement, snake_movment_input, spawn_food, spawn_snake};
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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FoodSpawnTimer::new(2.0))
        .insert_resource(SnakeMoveTimer::new(0.20))
        .insert_resource(SnakeBlocks::default())
        .insert_resource(LastBlockPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(
            Update,
            (
                snake_movment_input.before(snake_movement),
                snake_movement,
                snake_eating,
                snake_growth,
                size_scaling,
                position_translation,
                spawn_food,
                game_over,
            ),
        )
        .run();
}