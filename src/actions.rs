use crate::block::{Block, Orientation};
use crate::consts::*;
use crate::game_state::GameState;

impl GameState {
    pub fn hard_drop(&mut self) {
        self.try_translate(0, self.current_block.max_drop(&self.squares));
        self.update_timer = TICK_INTERVAL;
    }

    pub fn cache(&mut self) {
        if !self.used_hold {
            self.used_hold = true;
            let saved_current = self.current_block.blocktype;
            self.current_block = match self.held_block {
                Some(blocktype) => {
                    Block::new(blocktype, Orientation::Up).translate(X_SQUARES / 2, -5)
                }
                None => {
                    // reset the queue
                    if self.block_index == 14 {
                        self.update_queue();
                        self.block_index = 0;
                    }

                    let new_blocktype = TYPES[self.queue[self.block_index]];
                    self.block_index += 1;

                    Block::new(new_blocktype, Orientation::Up).translate(X_SQUARES / 2, -5)
                }
            };
            self.held_block = Some(saved_current);
        }
    }

    pub fn spin(&mut self) {
        let rotated = self.current_block.rotate();

        let overflow = self
            .current_block
            .rotate()
            .squares
            .iter()
            .fold(0, |over, square| {
                if square.pos.0 >= X_SQUARES && square.pos.0 - X_SQUARES + 1 > over {
                    square.pos.0 - X_SQUARES + 1
                } else if square.pos.0 < 0 && square.pos.0 < over {
                    square.pos.0
                } else {
                    over
                }
            });

        self.current_block = rotated.translate(-overflow, 0);
    }
}
