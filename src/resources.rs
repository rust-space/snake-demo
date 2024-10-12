use bevy::{prelude::{Deref, DerefMut, Entity, Resource}, time::{Timer, TimerMode}};

use crate::components::Position;

#[derive(Resource, Default, DerefMut, Deref)]
pub struct SnakeBlocks(pub Vec<Entity>);

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

impl SnakeMoveTimer {
    pub fn new() -> Self {
      Self(Timer::from_seconds(
        0.20,
        TimerMode::Repeating,
    ))
    }
}

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

impl FoodSpawnTimer {
    pub fn new() ->Self {
      Self(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    ))
    }
}

#[derive(Default, Resource)]
pub struct LastBlockPosition(pub Option<Position>);
