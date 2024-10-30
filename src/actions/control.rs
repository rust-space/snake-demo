use bevy::prelude::{ButtonInput, KeyCode, Res};

use crate::snake::SnakeHead;

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameControl {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    pub fn pressed(keyboard_input: &Res<ButtonInput<KeyCode>>, head : &SnakeHead) -> Self {
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            GameControl::Left
        } else if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::KeyD)
        {
            GameControl::Right
        } else if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            GameControl::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp)
        || keyboard_input.pressed(KeyCode::KeyW){
            GameControl::Up
        } else {
            head.direction
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
