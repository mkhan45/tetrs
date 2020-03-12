use ggez::graphics::{Color, Rect};

use crate::SCREEN_HEIGHT;
use crate::SQUARE_SIZE;

use crate::X_SQUARES;
use crate::Y_SQUARES;

use crate::BORDER_SIZE;

use std::convert::TryInto;

/// A square that is or was part of a block
#[derive(Clone, Copy, Debug)]
pub struct Square {
    /// Graphical component of the square
    pub rect: Rect,
    /// Logical position of the square
    pub pos: (i8, i8),
    pub color: Color,
}

impl Square {
    /// creates an identical square at the bottom of the screen
    fn bottom(x: i8) -> Self {
        Square {
            rect: Rect::new(f32::from(x) * SQUARE_SIZE, SCREEN_HEIGHT, 5., 5.),
            pos: (x, Y_SQUARES),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    /// finds the maximum distance a square could fall
    pub fn max_y_translate(&self, board: &[Square]) -> i8 {
        // starts by filtering the board to only squares on the same x axis, and then
        // looks down
        let max_square = board
            .iter()
            .filter(|square| square.pos.0 == self.pos.0 && square.pos.1 >= self.pos.1)
            .fold(Square::bottom(self.pos.0), |max_square, current_square| {
                if current_square.pos.1 <= max_square.pos.1 {
                    *current_square
                } else {
                    max_square
                }
            });
        max_square.pos.1 - self.pos.1
    }

    /// returns a translated version of the square
    pub fn translate(&self, x: i8, y: i8) -> Square {
        let mut new_rect = self.rect;
        new_rect.translate([f32::from(x) * SQUARE_SIZE, f32::from(y) * SQUARE_SIZE]);
        Square {
            rect: new_rect,
            pos: (self.pos.0 + x, self.pos.1 + y),
            color: self.color,
        }
    }

    fn new(x: i8, y: i8, color: Color) -> Self {
        Square {
            rect: Rect::new(
                f32::from(x) * SQUARE_SIZE + BORDER_SIZE,
                f32::from(y) * SQUARE_SIZE + BORDER_SIZE,
                SQUARE_SIZE - (BORDER_SIZE * 2.),
                SQUARE_SIZE - (BORDER_SIZE * 2.),
            ),
            pos: (x, y),
            color,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    Line,
    Square,
    L,
    ReverseL,
    T,
    S,
    Z,
}

/// A full block, in practice this is only used
/// as the selected block or in the queue/cache
/// since once a block is placed it becomes just
/// squares on the board
#[derive(Clone)]
pub struct Block {
    pub squares: [Square; 4],
    pub blocktype: BlockType,
    pub orientation: Orientation,
}

pub fn color(blocktype: BlockType) -> Color {
    match blocktype {
        BlockType::Line => Color::new(0.2, 0.2, 0.8, 1.0),
        BlockType::Square => Color::new(0.1, 1.0, 0.15, 1.0),
        BlockType::L => Color::new(0.05, 0.05, 1.0, 1.0),
        BlockType::ReverseL => Color::new(1.0, 0.25, 0.25, 1.0),
        BlockType::S => Color::new(0.1, 1.0, 0.1, 1.0),
        BlockType::Z => Color::new(1.0, 0.1, 0.1, 1.0),
        BlockType::T => Color::new(0.2, 0.05, 0.9, 1.0),
    }
}

impl Block {
    pub fn new(blocktype: BlockType, orientation: Orientation) -> Self {
        match (blocktype, orientation) {
            (BlockType::Line, Orientation::Up) => Block {
                squares: (0..4)
                    .map(|y_index| Square::new(0, y_index, color(blocktype)))
                    .collect::<Vec<Square>>()
                    .as_slice()
                    .try_into()
                    .unwrap(),
                blocktype,
                orientation,
            },
            (BlockType::Line, Orientation::Left) => Block {
                squares: (0..4)
                    .map(|x_index| Square::new(x_index, 0, color(blocktype)))
                    .collect::<Vec<Square>>()
                    .as_slice()
                    .try_into()
                    .unwrap(),
                blocktype,
                orientation,
            },
            (BlockType::Square, Orientation::Up) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::L, Orientation::Up) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(0, 2, color(blocktype)),
                    Square::new(1, 2, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::L, Orientation::Right) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(2, 0, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::L, Orientation::Down) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(1, 2, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::L, Orientation::Left) => Block {
                squares: [
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(2, 1, color(blocktype)),
                    Square::new(2, 0, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::ReverseL, Orientation::Up) => Block {
                squares: [
                    Square::new(1, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(1, 2, color(blocktype)),
                    Square::new(0, 2, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::ReverseL, Orientation::Right) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(2, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::ReverseL, Orientation::Down) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(0, 2, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::ReverseL, Orientation::Left) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(2, 0, color(blocktype)),
                    Square::new(2, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::S, Orientation::Up) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(1, 2, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::S, Orientation::Left) => Block {
                squares: [
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(2, 0, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::Z, Orientation::Up) => Block {
                squares: [
                    Square::new(1, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(0, 2, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::Z, Orientation::Left) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(2, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::T, Orientation::Up) => Block {
                squares: [
                    Square::new(1, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(2, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::T, Orientation::Right) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                    Square::new(0, 2, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::T, Orientation::Down) => Block {
                squares: [
                    Square::new(0, 0, color(blocktype)),
                    Square::new(1, 0, color(blocktype)),
                    Square::new(2, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },
            (BlockType::T, Orientation::Left) => Block {
                squares: [
                    Square::new(1, 0, color(blocktype)),
                    Square::new(1, 1, color(blocktype)),
                    Square::new(1, 2, color(blocktype)),
                    Square::new(0, 1, color(blocktype)),
                ],

                blocktype,
                orientation,
            },

            _ => {
                panic! {"invalid block, blocktype: {:?}, orientation: {:?}", blocktype, orientation}
            }
        }
    }

    pub fn rotate(&self) -> Block {
        match (self.blocktype, self.orientation){
            (BlockType::Line, Orientation::Left) => Block::new(BlockType::Line, Orientation::Up).translate(2, 0),
            (BlockType::Line, Orientation::Up) => Block::new(BlockType::Line, Orientation::Left).translate(-2, 0),
            (BlockType::Square, Orientation::Up) => Block::new(BlockType::Square, Orientation::Up),
            (BlockType::L, Orientation::Up) => Block::new(BlockType::L, Orientation::Right),
            (BlockType::L, Orientation::Right) => Block::new(BlockType::L, Orientation::Down),
            (BlockType::L, Orientation::Down) => Block::new(BlockType::L, Orientation::Left),
            (BlockType::L, Orientation::Left) => Block::new(BlockType::L, Orientation::Up),
            (BlockType::ReverseL, Orientation::Up) => Block::new(BlockType::ReverseL, Orientation::Right),
            (BlockType::ReverseL, Orientation::Right) => Block::new(BlockType::ReverseL, Orientation::Down),
            (BlockType::ReverseL, Orientation::Down) => Block::new(BlockType::ReverseL, Orientation::Left).translate(-1, 0),
            (BlockType::ReverseL, Orientation::Left) => Block::new(BlockType::ReverseL, Orientation::Up),
            (BlockType::S, Orientation::Up) => Block::new(BlockType::S, Orientation::Left),
            (BlockType::S, Orientation::Left) => Block::new(BlockType::S, Orientation::Up),
            (BlockType::Z, Orientation::Up) => Block::new(BlockType::Z, Orientation::Left),
            (BlockType::Z, Orientation::Left) => Block::new(BlockType::Z, Orientation::Up).translate(-1, 0),
            (BlockType::T, Orientation::Up) => Block::new(BlockType::T, Orientation::Right),
            (BlockType::T, Orientation::Right) => Block::new(BlockType::T, Orientation::Down).translate(-1, 0),
            (BlockType::T, Orientation::Down) => Block::new(BlockType::T, Orientation::Left),
            (BlockType::T, Orientation::Left) => Block::new(BlockType::T, Orientation::Up).translate(-1, 0),
            _ => {panic!{"invalid block, blocktype: {:?}, orientation: {:?}", self.blocktype, self.orientation}}
        }.translate(self.squares[0].pos.0, self.squares[1].pos.1)
    }

    pub fn translate(&self, x: i8, y: i8) -> Block {
        let mut cloned = self.squares;
        let cloned: Vec<Square> = cloned
            .iter_mut()
            .map(|square| square.translate(x, y))
            .collect();
        Block {
            squares: cloned.as_slice().try_into().unwrap(),
            orientation: self.orientation,
            blocktype: self.blocktype,
        }
    }

    pub fn overlaps(&self, board: &[Square]) -> bool {
        self.squares.iter().any(|square| {
            board
                .iter()
                .any(|other_square| square.pos == other_square.pos)
        })
    }

    pub fn is_valid(&self, board: &[Square]) -> bool {
        !self.overlaps(board)
            && !self.squares.iter().any(|&square| {
                square.pos.0 < 0 || square.pos.0 >= X_SQUARES || square.pos.1 >= Y_SQUARES
            })
    }

    pub fn max_drop(&self, board: &[Square]) -> i8 {
        self.squares.iter().fold(Y_SQUARES + 5, |max_dist, square| {
            let square_max = square.max_y_translate(board);
            if square_max < max_dist {
                square_max
            } else {
                max_dist
            }
        }) - 1
    }
}
