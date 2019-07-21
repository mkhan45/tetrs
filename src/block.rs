use ggez::graphics::{Color, Rect};

use crate::SQUARE_SIZE;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::X_SQUARES;
use crate::Y_SQUARES;
const COLL_SIZE: f32 = SQUARE_SIZE - 0.05;

use std::convert::TryInto;


#[derive(Clone, Copy)]
pub struct Square {
    pub rect: Rect,
    pub color: Color,
}

impl Square{
    fn bottom(x: f32) -> Self{
        Square{
            rect: Rect::new(x, SCREEN_HEIGHT, COLL_SIZE, COLL_SIZE),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    fn board_pos(&self) -> (i32, i32){
        ((self.rect.x / SQUARE_SIZE) as i32, (self.rect.y / SQUARE_SIZE) as i32)
    }

    pub fn max_y_translate(&self, board: &Vec<Square>) -> i16 {
        let max_square = board.iter().filter(|square| square.rect.x == self.rect.x)
            .fold(Square::bottom(self.rect.x), |max_square, current_square|{
                println!("{}", max_square.board_pos().1);
                if current_square.rect.y <= max_square.rect.y { *current_square } else { max_square }
            });
        // println!("{}", max_square.board_pos().1);
        (max_square.board_pos().1 - self.board_pos().1 - 1).try_into().unwrap()
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
                squares: (0..4).map(|y_index| y_index as f32 * COLL_SIZE).map(|y_pos| {
                    Square{
                        rect: Rect::new(0., y_pos, COLL_SIZE, COLL_SIZE),
                        color: Color::new(0.1, 0.1, 1.0, 1.0),
                    }
                }).collect(),
                blocktype,
                orientation,
            },
            (BlockType::Square, Orientation::Up) => Block{
                squares: vec![
                    Square{rect: Rect::new(0., 0., COLL_SIZE, COLL_SIZE),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(0., SQUARE_SIZE, COLL_SIZE, COLL_SIZE),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(SQUARE_SIZE, 0., COLL_SIZE, COLL_SIZE),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(SQUARE_SIZE, SQUARE_SIZE, COLL_SIZE, COLL_SIZE),
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
        }
    }

    pub fn translate(&self, x: i16, y: i16) -> Block{
        let mut cloned = self.squares.clone();
        cloned.iter_mut().for_each(|square|{
            square.rect.translate([x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE]);
        });
        Block{
            squares: cloned,
            orientation: self.orientation,
            blocktype: self.blocktype,
        }
    }

    pub fn overlaps(&self, board: &Vec<Square>) -> bool{
        self.squares.iter().any(|square| board.iter().any(|other_square| square.rect.overlaps(&other_square.rect)))
    }

    pub fn is_valid(&self, board: &Vec<Square>) -> bool{
        !self.overlaps(board) && !self.squares.iter().any(|&square| {
            square.rect.x <= -0.05 || square.rect.x + COLL_SIZE >= SCREEN_WIDTH 
                || square.rect.y + COLL_SIZE >= SCREEN_HEIGHT
        })
    }

    pub fn min_square(&self, x: i32) -> Square{
        self.squares.iter().filter(|square| square.board_pos().0 == x)
            .fold(self.squares[0], |min_square, current_square|{
                if current_square.rect.y > min_square.rect.y { *current_square } else { min_square }
            })
    }

    pub fn max_drop(&self, board: &Vec<Square>) -> i16 {
        let (min_x, max_x) = self.squares.iter().fold((0i32, X_SQUARES as i32), |(min, max), current|{
            let current_x = current.board_pos().0;
            if current_x < min { (current_x, max) }
            else if current_x > max { (min, current_x) }
            else { (min, max) }
        });

        (min_x..max_x).fold(Y_SQUARES as i16, |max_dist, x| {
            let square_max = self.min_square(x).max_y_translate(board);
            if square_max < max_dist { square_max } else { max_dist }
        })
    }
}
