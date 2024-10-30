use crate::actions::{Position, Size};
use crate::{GameState, ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR};
use bevy::prelude::*;
use rand::random;


pub struct FoodPlugin;

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

impl FoodSpawnTimer {
    pub fn new(d: f32) ->Self {
      Self(Timer::from_seconds(
        d,
        TimerMode::Repeating,
    ))
    }
}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoodSpawnTimer::new(2.0))
        .add_systems(Update,spawn_food.run_if(in_state(GameState::Playing)));
    }
}

pub fn spawn_food(mut cmds: Commands, time: Res<Time>, mut timer: ResMut<FoodSpawnTimer>) {
  if !timer.0.tick(time.delta()).finished() {
      return;
  }
  cmds.spawn(SpriteBundle {
      sprite: Sprite {
          color: FOOD_COLOR,
          ..default()
      },
      ..default()
  })
  .insert(Food)
  .insert(Position {
      x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
      y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
  })
  .insert(Size::square(0.8));
}