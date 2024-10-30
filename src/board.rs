use bevy::prelude::*;

use crate::GameState;

pub struct BoardPlugin;

#[derive(Component)]
pub struct Board;

#[derive(Default, Resource)]
pub struct Score(u32);

#[derive(Event)]
pub struct AddScoreEvent;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_event::<AddScoreEvent>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, update_board)
            .add_systems(OnEnter(GameState::GameOver), reset_score);
    }
}

impl Score {
    fn reset(&mut self) {
        self.0 = 0;
    }

    fn increment(&mut self) {
        self.0 += 1;
    }

    fn get(&self) -> u32 {
        self.0
    }
}

fn setup_board(mut commands: Commands) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(Board);
}

fn update_board(
    mut score: ResMut<Score>,
    mut reader: EventReader<AddScoreEvent>,
    mut query: Query<&mut Text, With<Board>>,
) {
    if reader.read().next().is_some() {
        score.increment();
    }

    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", score.get());
    }
}

fn reset_score(mut score: ResMut<Score>, mut query: Query<&mut Text, With<Board>>) {
    for mut text in query.iter_mut() {
        score.reset();
        text.sections[1].value = format!("{}", score.get());
    }
}
