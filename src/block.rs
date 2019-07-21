use ggez::graphics::{Color, Rect};

use crate::SQUARE_SIZE;


#[derive(Clone, Copy)]
pub struct Square {
    pub rect: Rect,
    pub color: Color,
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
        let coll_size = SQUARE_SIZE - 0.05;
        match (blocktype, orientation) {
            (BlockType::Line, Orientation::Up) => Block {
                squares: (0..4).map(|y_index| y_index as f32 * coll_size).map(|y_pos| {
                    Square{
                        rect: Rect::new(0., y_pos, coll_size, coll_size),
                        color: Color::new(0.1, 0.1, 1.0, 1.0),
                    }
                }).collect(),
                blocktype,
                orientation,
            },
            (BlockType::Square, Orientation::Up) => Block{
                squares: vec![
                    Square{rect: Rect::new(0., 0., coll_size, coll_size),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(0., SQUARE_SIZE, coll_size, coll_size),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(SQUARE_SIZE, 0., coll_size, coll_size),
                    color: Color::new(0.1, 1.0, 0.15, 1.0)},

                    Square{rect: Rect::new(SQUARE_SIZE, SQUARE_SIZE, coll_size, coll_size),
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

    pub fn translate(&self, x: u16, y: u16) -> Block{
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

    pub fn overlaps(&self, board: Vec<Square>) -> bool{
        self.squares.iter().any(|square| board.iter().any(|other_square| square.rect.overlaps(&other_square.rect)))
    }
}
