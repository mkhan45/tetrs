use crate::block::BlockType;

pub const SCREEN_HEIGHT: f32 = 600.;
pub const SCREEN_WIDTH: f32 = 300.;
pub const SCREEN_WIDTHER: f32 = SCREEN_WIDTH * 1.7;

pub const X_SQUARES: i8 = 10;
pub const Y_SQUARES: i8 = 20;

pub const SQUARE_SIZE: f32 = SCREEN_HEIGHT / Y_SQUARES as f32;

pub const BORDER_SIZE: f32 = 0.5;

pub const TICK_INTERVAL: usize = 60;

pub const INPUT_INTERVAL: u16 = 5;
pub const INPUT_REPEAT_DELAY: u16 = 8;

pub const TYPES: [BlockType; 7] = [
    BlockType::Line,
    BlockType::Square,
    BlockType::L,
    BlockType::ReverseL,
    BlockType::S,
    BlockType::Z,
    BlockType::T,
];
