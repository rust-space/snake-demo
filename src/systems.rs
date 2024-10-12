use bevy::{input::ButtonInput, math::Vec3, prelude::{default, Camera2dBundle, Commands, Entity, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Transform, With}, sprite::{Sprite, SpriteBundle}, time::Time, window::Window};
use rand::random;

use crate::{components::{Direction, Food, Position, Size, SnakeBlock, SnakeHead}, events::{GameOverEvent, GrowthEvent}, resources::{FoodSpawnTimer, LastBlockPosition, SnakeBlocks, SnakeMoveTimer}, ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR, SNAKE_BODY_COLOR, SNAKE_HEAD_COLOR};

pub fn setup_camera(mut cmds: Commands) {
  cmds.spawn(Camera2dBundle::default());
}

pub fn spawn_snake(mut cmds: Commands, mut blocks: ResMut<SnakeBlocks>) {
  *blocks = SnakeBlocks(vec![
      cmds.spawn(SpriteBundle {
          sprite: Sprite {
              color: SNAKE_HEAD_COLOR,
              ..default()
          },
          transform: Transform {
              scale: Vec3::new(10.0, 10.0, 10.0),
              ..default()
          },
          ..default()
      })
      .insert(SnakeHead {
          direction: Direction::Up,
      })
      .insert(SnakeBlock)
      .insert(Position { x: 3, y: 3 })
      .insert(Size::square(0.8))
      .id(),
      spawn_block(cmds, Position { x: 3, y: 2 }),
  ]);
}

pub fn snake_movment_input(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut heads: Query<&mut SnakeHead>,
) {
  if let Some(mut head) = heads.iter_mut().next() {
      let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft)
          || keyboard_input.pressed(KeyCode::KeyJ)
      {
          Direction::Left
      } else if keyboard_input.pressed(KeyCode::ArrowRight)
          || keyboard_input.pressed(KeyCode::KeyL)
      {
          Direction::Right
      } else if keyboard_input.pressed(KeyCode::ArrowDown)
          || keyboard_input.pressed(KeyCode::KeyK)
      {
          Direction::Down
      } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyI)
      {
          Direction::Up
      } else {
          head.direction
      };
      // 按运行反方向不生效
      if dir != head.direction.opposite() {
          head.direction = dir;
      }
  }
}

pub fn snake_movement(
  mut heads: Query<(Entity, &mut SnakeHead)>,
  blocks: ResMut<SnakeBlocks>,
  mut positions: Query<&mut Position>,
  mut last_block_pos: ResMut<LastBlockPosition>,
  time: Res<Time>,
  mut timer: ResMut<SnakeMoveTimer>,
  mut game_over_writer: EventWriter<GameOverEvent>,
) {
  if !timer.0.tick(time.delta()).finished() {
      return;
  }

  if let Some((entity, head)) = heads.iter_mut().next() {
      let block_positions = blocks
          .iter()
          .map(|e| *positions.get_mut(*e).unwrap())
          .collect::<Vec<Position>>();
      // 蛇头的移动
      let mut head_pos = positions.get_mut(entity).unwrap();
      match &head.direction {
          Direction::Left => {
              head_pos.x -= 1;
          }
          Direction::Right => {
              head_pos.x += 1;
          }
          Direction::Up => {
              head_pos.y += 1;
          }
          Direction::Down => {
              head_pos.y -= 1;
          }
      };
      // 撞墙判断
      if head_pos.x < 0
          || head_pos.y < 0
          || head_pos.x as u32 >= ARENA_WIDTH
          || head_pos.y as u32 >= ARENA_HEIGHT
      {
          game_over_writer.send(GameOverEvent);
      }
      // 撞自身判断
      if block_positions.contains(&head_pos) {
          game_over_writer.send(GameOverEvent);
      }
      // 蛇身的移动
      block_positions
          .iter()
          .zip(blocks.iter().skip(1))
          .for_each(|(pos, block)| {
              *positions.get_mut(*block).unwrap() = *pos;
          });
      *last_block_pos = LastBlockPosition(Some(*block_positions.last().unwrap()))
  }
}

pub fn snake_eating(
  mut cmds: Commands,
  mut growth_writer: EventWriter<GrowthEvent>,
  food_positions: Query<(Entity, &Position), With<Food>>,
  head_positions: Query<&Position, With<SnakeHead>>,
) {
  for head_pos in head_positions.iter() {
      for (entity, food_pos) in food_positions.iter() {
          // 实物和头部位置重合，说明吃到实物
          if food_pos == head_pos {
              cmds.entity(entity).despawn();
              growth_writer.send(GrowthEvent);
          }
      }
  }
}

pub fn snake_growth(
  cmds: Commands,
  last_block_pos: Res<LastBlockPosition>,
  mut blocks: ResMut<SnakeBlocks>,
  mut growth_reader: EventReader<GrowthEvent>,
) {
  if growth_reader.read().next().is_some() {
      blocks.push(spawn_block(cmds, last_block_pos.0.unwrap()));
  }
}

fn spawn_block(mut cmds: Commands, pos: Position) -> Entity {
  cmds.spawn(SpriteBundle {
      sprite: Sprite {
          color: SNAKE_BODY_COLOR,
          ..default()
      },
      ..default()
  })
  .insert(SnakeBlock)
  .insert(pos)
  .insert(Size::square(0.65))
  .id()
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

// 计算方块元素的大小
pub fn size_scaling(
  primary_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
  mut q: Query<(&Size, &mut Transform)>,
) {
  let window = primary_query.get_single().unwrap();
  for (sprite_size, mut transform) in q.iter_mut() {
      transform.scale = Vec3::new(
          sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
          sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
          1.0,
      )
  }
}

// 计算位移
pub fn position_translation(
  primary_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
  mut q: Query<(&Position, &mut Transform)>,
) {
  fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
      let block_size = bound_window / bound_game;
      pos / bound_game * bound_window - (bound_window / 2.0) + (block_size / 2.0)
  }

  let window = primary_query.get_single().unwrap();
  for (pos, mut transform) in q.iter_mut() {
      transform.translation = Vec3::new(
          convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
          convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
          0.0,
      );
  }
}

// 游戏结束
pub fn game_over(
  mut cmds: Commands,
  mut reader: EventReader<GameOverEvent>,
  blocks_res: ResMut<SnakeBlocks>,
  food: Query<Entity, With<Food>>,
  blocks: Query<Entity, With<SnakeBlock>>,
) {
  if reader.read().next().is_some() {
      for entity in food.iter().chain(blocks.iter()) {
          cmds.entity(entity).despawn();
      }
      spawn_snake(cmds, blocks_res);
  }
}