use ggez::graphics::{Color, Rect};

use crate::SQUARE_SIZE;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::X_SQUARES;
use crate::Y_SQUARES;


#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub rect: Rect,
    pub pos: (isize, isize),
    pub color: Color,
}

impl Square{
    fn bottom(x: isize) -> Self{
        Square{
            rect: Rect::new(x as f32 * SQUARE_SIZE, SCREEN_HEIGHT, SQUARE_SIZE, SQUARE_SIZE),
            pos: (x, Y_SQUARES),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn max_y_translate(&self, board: &[Square]) -> isize {
        let max_square = board.iter().filter(|square| square.pos.0 == self.pos.0)
            .fold(Square::bottom(self.pos.0), |max_square, current_square|{
                if current_square.pos.1 <= max_square.pos.1 { *current_square } else { max_square }
            });
        max_square.pos.1 - self.pos.1
    }

    pub fn translate(&self, x: isize, y: isize) -> Square {
        let mut new_rect = self.rect;
        new_rect.translate([x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE]);
        Square{
            rect: new_rect,
            pos: ((self.pos.0 as isize + x) as isize, (self.pos.1 as isize + y) as isize),
            color: self.color,
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

#[derive(Clone)]
pub struct Block {
    pub squares: Vec<Square>,
    pub blocktype: BlockType,
    pub orientation: Orientation,
}

impl Block {
    pub fn new(blocktype: BlockType, orientation: Orientation) -> Self {
        match (blocktype, orientation) {
            (BlockType::Line, Orientation::Up) => Block {
                squares: (0..4).map(|y_index|{
                    Square{
                        rect: Rect::new(0., y_index as f32 * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE),
                        pos: (0, y_index),
                        color: Color::new(0.1, 0.1, 1.0, 1.0),
                    }
                }).collect(),
                blocktype,
                orientation,
            },
            (BlockType::Line, Orientation::Left) => Block {
                squares: (0..4).map(|x_index|{
                    Square{
                        rect: Rect::new(x_index as f32 * SQUARE_SIZE, 0., SQUARE_SIZE, SQUARE_SIZE),
                        pos: (x_index, 0),
                        color: Color::new(0.1, 0.1, 1.0, 1.0),
                    }
                }).collect(),
                blocktype,
                orientation,
            },
            (BlockType::Square, Orientation::Up) => Block{
                squares: vec![
                    Square{rect: Rect::new(0., 0., SQUARE_SIZE, SQUARE_SIZE),
                    pos: (0, 0),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(0., SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE),
                    pos: (0, 1),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(SQUARE_SIZE, 0., SQUARE_SIZE, SQUARE_SIZE),
                    pos: (1, 0),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE),
                    pos: (1, 1),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},
                ],
                blocktype,
                orientation,
            },
            _ => {panic!{"invalid block, blocktype: {:?}, orientation: {:?}", blocktype, orientation}}
        }
    }

    pub fn rotate(&self) -> Block{ //there's probably a better way to do this
        match (self.blocktype, self.orientation){
            (BlockType::Line, Orientation::Left) => Block::new(BlockType::Line, Orientation::Up),
            (BlockType::Line, Orientation::Up) => Block::new(BlockType::Line, Orientation::Left),
            (BlockType::Square, Orientation::Up) => Block::new(BlockType::Square, Orientation::Up),
            _ => {panic!{"invalid block, blocktype: {:?}, orientation: {:?}", self.blocktype, self.orientation}}
        }.translate(self.squares[0].pos.0, self.squares[1].pos.1)
    }

    pub fn translate(&self, x: isize, y: isize) -> Block{
        let mut cloned = self.squares.clone();
        let cloned: Vec<Square> = cloned.iter_mut().map(|square|{
            square.translate(x, y)
        }).collect();
        Block{
            squares: cloned,
            orientation: self.orientation,
            blocktype: self.blocktype,
        }
    }

    pub fn overlaps(&self, board: &[Square]) -> bool{
        self.squares.iter().any(|square| board.iter().any(|other_square| square.pos == other_square.pos))
    }

    pub fn is_valid(&self, board: &[Square]) -> bool{
        !self.overlaps(board) && !self.squares.iter().any(|&square| {
            square.pos.0 < 0 || square.pos.0 >= X_SQUARES
                || square.pos.1 >= Y_SQUARES
        })
    }

    pub fn min_square(&self, x: isize) -> Square{
        self.squares.iter().filter(|square| square.pos.0 == x)
            .fold(self.squares[0], |min_square, current_square|{
                if current_square.pos.1 > min_square.pos.1 { *current_square } else { min_square }
            })
    }

    pub fn max_drop(&self, board: &[Square]) -> isize {
        let (min_x, max_x) = self.squares.iter().fold((X_SQUARES, 0), |(min, max), current|{
            let current_x = current.pos.0;
            if current_x < min { (current_x, max) }
            else if current_x > max { (min, current_x) }
            else { (min, max) }
        });

        let potential_max = (min_x..=max_x).fold(Y_SQUARES, |max_dist, x| {
            let square_max = self.min_square(x).max_y_translate(board);
            if square_max < max_dist { square_max } else { max_dist }
        });

        for i in (0..potential_max).rev(){
            if self.translate(0, i).is_valid(board) { return i };
        }
        return 0;
    }
}
