use crate::actions::control::GameControl;
use crate::actions::{Position, Size};
use crate::board::AddScoreEvent;
use crate::food::Food;
use crate::{GameState, ARENA_HEIGHT, ARENA_WIDTH, SNAKE_BODY_COLOR, SNAKE_HEAD_COLOR};
use bevy::prelude::*;

pub struct SnakePlugin;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: GameControl,
}

#[derive(Component)]
pub struct SnakeBlock;

#[derive(Resource, Default, DerefMut, Deref)]
pub struct SnakeBlocks(pub Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastBlockPosition(pub Option<Position>);

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Event)]
pub struct GrowthEvent;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeMoveTimer::new(0.20))
            .insert_resource(SnakeBlocks::default())
            .insert_resource(LastBlockPosition::default())
            .add_event::<GrowthEvent>()
            .add_systems(OnEnter(GameState::Playing), spawn_snake)
            .add_systems(
                Update,
                (
                    snake_movment_input
                        .run_if(in_state(GameState::Playing))
                        .before(move_snake),
                    move_snake.run_if(in_state(GameState::Playing)),
                    snake_eating.run_if(in_state(GameState::Playing)),
                    snake_growth.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}

impl SnakeMoveTimer {
    pub fn new(d: f32) -> Self {
        Self(Timer::from_seconds(d, TimerMode::Repeating))
    }
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
            direction: GameControl::default(),
        })
        .insert(SnakeBlock)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8))
        .id(),
        spawn_block(cmds, Position { x: 3, y: 2 }),
    ]);
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

pub fn move_snake(
    mut heads: Query<(Entity, &mut SnakeHead)>,
    blocks: ResMut<SnakeBlocks>,
    mut positions: Query<&mut Position>,
    mut last_block_pos: ResMut<LastBlockPosition>,
    time: Res<Time>,
    mut timer: ResMut<SnakeMoveTimer>,
    mut next_state: ResMut<NextState<GameState>>,
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
            GameControl::Left => {
                head_pos.x -= 1;
            }
            GameControl::Right => {
                head_pos.x += 1;
            }
            GameControl::Up => {
                head_pos.y += 1;
            }
            GameControl::Down => {
                head_pos.y -= 1;
            }
        };
        // 撞墙判断
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            next_state.set(GameState::GameOver);
        }
        // 撞自身判断
        if block_positions.contains(&head_pos) {
            next_state.set(GameState::GameOver);
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

pub fn snake_movment_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: GameControl = GameControl::pressed(&keyboard_input, &head);
        // 按运行反方向不生效
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_eating(
    mut cmds: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    mut score_writer: EventWriter<AddScoreEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (entity, food_pos) in food_positions.iter() {
            // 实物和头部位置重合，说明吃到实物
            if food_pos == head_pos {
                cmds.entity(entity).despawn();
                score_writer.send(AddScoreEvent);
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
