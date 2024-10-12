use bevy::{color::Color, window::WindowResolution};

pub mod components;
pub mod resources;
pub mod events;
pub mod systems;

pub const ARENA_WIDTH: u32 = 25;
pub const ARENA_HEIGHT: u32 = 25;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.04, 0.04, 0.04);
pub const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
pub const SNAKE_BODY_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

// 设置分辨率
pub fn setup_resolution() -> WindowResolution {
  WindowResolution::new(500.0, 500.0)
}