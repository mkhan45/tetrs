use crate::block::*;
use crate::input::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::consts::*;
use crate::main_state::{Signal, SignalState, StateTrait};
use crate::menu_state::GameOverData;

use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawMode, DrawParam, Font, MeshBuilder, Scale, Text, TextFragment},
    input::keyboard::KeyCode,
    timer, Context, GameResult,
};

#[derive(Clone)]
pub struct GameState {
    pub squares: Vec<Square>,
    pub inputs: Rc<RefCell<HashMap<InputAction, InputState>>>,
    pub current_block: Block,
    pub update_timer: usize,
    pub held_block: Option<BlockType>,
    pub queue: [usize; 14],
    pub block_index: usize,
    pub used_hold: bool,
    pub queued_queue: [usize; 14],
    pub lines: usize,
    pub font: Font,
    pub info_text: Text,
    pub signals: Vec<Signal>,
}

pub fn generate_queue() -> [usize; 14] {
    let mut rng = thread_rng();

    //generates an iterator [0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6]
    let mut slice = (0..=6).cycle().take(14).collect::<Vec<usize>>();
    slice.shuffle(&mut rng);
    slice.as_slice().try_into().expect("error generating queue")
}

impl GameState {
    pub fn new(font: Font) -> Self {
        // makes squares a vector with capacity height * width
        let squares = Vec::with_capacity(
            (i16::from(X_SQUARES) * i16::from(Y_SQUARES))
                .try_into()
                .unwrap(),
        );

        // creates current block at top center of board
        let current_block =
            Block::new(BlockType::Line, Orientation::Up).translate(X_SQUARES as i8 / 2, 0);

        // initializes input states
        let inputs = [
            (InputAction::Spin, InputState::default()),
            (InputAction::SoftDrop, InputState::default()),
            (InputAction::HardDrop, InputState::default()),
            (InputAction::MoveLeft, InputState::default()),
            (InputAction::MoveRight, InputState::default()),
            (InputAction::Cache, InputState::default()),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<InputAction, InputState>>();

        let info_text = Text::new(
            TextFragment::new("Lines: 0 \n 0:00")
                .font(font)
                .scale(Scale::uniform(24.0)),
        );

        GameState {
            squares,
            inputs: Rc::new(RefCell::new(inputs)),
            current_block,
            update_timer: 0,
            held_block: None,
            queue: generate_queue(),
            block_index: 0,
            used_hold: false,
            queued_queue: generate_queue(),
            lines: 0,
            info_text,
            font,
            signals: Vec::new(),
        }
    }

    /// tries to translate selected block by x and y
    pub fn try_translate(&mut self, x: i8, y: i8) {
        let translated = self.current_block.translate(x, y);
        if translated.is_valid(&self.squares) {
            self.current_block = translated;
        }
    }

    /// swaps current queue with next queue
    /// I used two queues because the graphics need to be continuous
    pub fn update_queue(&mut self) {
        let mut rng = thread_rng();
        self.queue = self.queued_queue;
        self.queued_queue.shuffle(&mut rng);
    }

    /// updates all input states
    pub fn update_inputs(&mut self, ctx: &Context) {
        use ggez::input::keyboard::pressed_keys;
        pressed_keys(&ctx)
            .iter()
            .filter_map(|keycode| match keycode {
                KeyCode::Up | KeyCode::W => Some(InputAction::Spin),
                KeyCode::Left | KeyCode::A => Some(InputAction::MoveLeft),
                KeyCode::Right | KeyCode::D => Some(InputAction::MoveRight),
                KeyCode::Down | KeyCode::S => Some(InputAction::SoftDrop),
                KeyCode::C => Some(InputAction::Cache),
                KeyCode::Space => Some(InputAction::HardDrop),
                _ => None,
            })
            .for_each(|action| {
                self.inputs
                    .borrow_mut()
                    .get_mut(&action)
                    .unwrap()
                    .set_pressed(true);
            });

        self.inputs
            .borrow_mut()
            .values_mut()
            .for_each(|input_state| {
                input_state.update();
            });
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update_timer += 1;

        self.update_inputs(ctx);
        self.inputs
            .clone()
            .borrow()
            .iter()
            .for_each(|(action, input_state)| {
                if input_state.repeated(INPUT_REPEAT_DELAY, INPUT_INTERVAL) {
                    match *action {
                        InputAction::MoveLeft => self.try_translate(-1, 0),
                        InputAction::MoveRight => self.try_translate(1, 0),
                        InputAction::SoftDrop => self.try_translate(0, 1),
                        InputAction::HardDrop => self.hard_drop(),
                        InputAction::Cache => self.cache(),
                        InputAction::Spin => self.spin(),
                    }
                }
            });

        // only update on ticks
        if self.update_timer >= TICK_INTERVAL {
            self.update_timer = 0;

            // create a temporary block that is the current_block translated down by one
            let translated = self.current_block.translate(0, 1);

            if translated.is_valid(&self.squares) {
                self.current_block = translated;
            } else {
                // update the queue if it's at the end
                if self.block_index == 14 {
                    self.update_queue();
                    self.block_index = 0;
                }

                let blocktype = TYPES[self.queue[self.block_index]];
                self.block_index += 1;

                // if any of the squares are over the top of the screen,
                // end the game
                if self
                    .current_block
                    .squares
                    .iter()
                    .any(|square| square.pos.1 < 0)
                {
                    self.signals.push(Signal::EndGame(GameOverData {
                        lines: self.lines,
                        time: timer::time_since_start(ctx),
                    }));
                };

                // since the block is not valid, it is colliding,
                // so place it on the board and set `used_hold`
                // to false
                self.squares
                    .append(&mut self.current_block.squares.to_vec());
                self.used_hold = false;

                // find and clear full rows or end the game at 40 lines
                {
                    let (min_y, max_y) = find_minmax(&self.current_block.squares);

                    (min_y..max_y + 1).for_each(|y| {
                        let row_cnt = self
                            .squares
                            .iter()
                            .filter(|square| square.pos.1 == y)
                            .count();

                        if row_cnt >= X_SQUARES.try_into().unwrap() {
                            self.lines += 1;

                            self.squares = clear_lines(&self.squares, y);
                        }
                    });
                }

                // reset the current block
                self.current_block = Block::new(blocktype, Orientation::Up)
                    .translate(X_SQUARES / 2, 0)
                    .translate(0, -5);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        if ggez::timer::ticks(ctx) % 60 == 0 {
            self.info_text = Text::new(
                TextFragment::new(format! {"Lines: {}\n{}", self.lines, duration_display(timer::time_since_start(ctx))})
                .font(self.font)
                .scale(Scale::uniform(24.0)),
            );
        }

        graphics::draw(
            ctx,
            &self.info_text,
            DrawParam::new().dest([SCREEN_WIDTH + 25., 10.]),
        )
        .expect("Error drawing info text");

        let mut mesh = MeshBuilder::new();

        self.current_block.squares.iter().for_each(|square| {
            mesh.rectangle(DrawMode::fill(), square.rect, square.color);
        });

        self.squares.iter().for_each(|square| {
            mesh.rectangle(DrawMode::fill(), square.rect, square.color);
        });

        {
            let preview = self
                .current_block
                .translate(0, self.current_block.max_drop(&self.squares));

            preview.squares.iter().for_each(|square| {
                mesh.rectangle(
                    DrawMode::fill(),
                    square.rect,
                    Color::new(1.0, 1.0, 1.0, 0.5),
                );
            });
        }

        for i in self.block_index..(self.block_index + 3) {
            let future_type = if i < 14 {
                TYPES[self.queue[i]]
            } else {
                TYPES[self.queued_queue[i - 13]]
            };
            let future_block = Block::new(future_type, Orientation::Up).translate(
                X_SQUARES + 2,
                (6 + 5 * (i - self.block_index)).try_into().unwrap(),
            );

            future_block.squares.iter().for_each(|square| {
                mesh.rectangle(DrawMode::fill(), square.rect, color(future_block.blocktype));
            });
        }

        if let Some(held_type) = self.held_block {
            let held = match held_type {
                BlockType::Line => {
                    Block::new(BlockType::Line, Orientation::Left).translate(X_SQUARES + 1, 3)
                }
                BlockType::Square => {
                    Block::new(BlockType::Square, Orientation::Up).translate(X_SQUARES + 3, 2)
                }
                BlockType::T => {
                    Block::new(BlockType::T, Orientation::Up).translate(X_SQUARES + 2, 2)
                }
                _ => Block::new(held_type, Orientation::Up).translate(X_SQUARES + 3, 2),
            };

            held.squares.iter().for_each(|square| {
                mesh.rectangle(DrawMode::fill(), square.rect, color(held_type));
            });
        }

        mesh.line(
            &[
                [SCREEN_WIDTH as f32, 0.],
                [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
            ],
            2.,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .unwrap();

        mesh.line(
            &[[SCREEN_WIDTH as f32, 150.], [SCREEN_WIDTHER as f32, 150.]],
            2.,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .unwrap();

        let mesh = &mesh.build(ctx).unwrap();

        graphics::draw(ctx, mesh, DrawParam::new()).unwrap();
        graphics::present(ctx).expect("error rendering");

        Ok(())
    }
}

impl SignalState for GameState {
    fn signals(&mut self) -> &mut Vec<Signal> {
        &mut self.signals
    }
}

impl StateTrait for GameState {}

/// find the min and max height of squares in the block,
/// used to find filled rows
fn find_minmax(squares: &[Square]) -> (i8, i8) {
    squares.iter().fold((0, Y_SQUARES), |(min, max), current| {
        let current_y = current.pos.1;
        if current_y < min {
            (current_y, max)
        } else if current_y > max {
            (min, current_y)
        } else {
            (min, max)
        }
    })
}

fn clear_lines(squares: &[Square], row_y: i8) -> Vec<Square> {
    squares
        .iter()
        .filter(|square| square.pos.1 != row_y)
        .map(|square| {
            if square.pos.1 < row_y {
                square.translate(0, 1)
            } else {
                *square
            }
        })
        .collect()
}

fn duration_display(duration: std::time::Duration) -> String {
    let (mins, secs) = (duration.as_secs() / 60, duration.as_secs() % 60);
    if secs < 10 {
        format!("{}:0{}", mins, secs)
    } else {
        format!("{}:{}", mins, secs)
    }
}
