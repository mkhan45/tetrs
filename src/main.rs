//#![feature(stmt_expr_attributes)] //for rustfmt
extern crate ggez;
use ggez::{
    event,
    event::EventHandler,
    graphics,
    graphics::{Color, DrawMode, DrawParam, MeshBuilder},
    input::keyboard,
    input::keyboard::KeyCode,
    timer, Context, GameResult,
};

use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

extern crate rand;

mod block;
use block::*;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 300.;
const SCREEN_WIDTHER: f32 = SCREEN_WIDTH * 1.7;

const X_SQUARES: i8 = 10;
const Y_SQUARES: i8 = 20;

const SQUARE_SIZE: f32 = SCREEN_HEIGHT / Y_SQUARES as f32;

const BORDER_SIZE: f32 = 0.5;

const TICK_INTERVAL: usize = 30;

static TYPES: [BlockType; 7] = [
    BlockType::Line,
    BlockType::Square,
    BlockType::L,
    BlockType::ReverseL,
    BlockType::S,
    BlockType::Z,
    BlockType::T,
];

#[derive(Clone)]
struct MainState {
    pub squares: Vec<Square>,
    pub current_block: Block,
    pub positions: Vec<Vec<(f32, f32)>>,
    pub update_timer: usize,
    pub held_block: Option<BlockType>,
    pub queue: [usize; 14],
    pub block_index: usize,
    pub used_hold: bool,
    pub queued_queue: [usize; 14],
    pub lines: usize,
}

pub fn generate_queue() -> [usize; 14] {
    let mut rng = thread_rng();
    let mut slice = (0..=6).chain(0..=6).collect::<Vec<usize>>();
    slice.shuffle(&mut rng);
    slice.as_slice().try_into().unwrap()
}

impl MainState {
    fn new() -> Self {
        let squares = Vec::with_capacity((X_SQUARES as i16 * Y_SQUARES as i16).try_into().unwrap());

        let current_block =
            Block::new(BlockType::Line, Orientation::Up).translate(X_SQUARES as i8 / 2, 0);

        let positions: Vec<Vec<(f32, f32)>> = (0..X_SQUARES)
            .map(|x_index| {
                (0..Y_SQUARES)
                    .map(|y_index| {
                        (
                            f32::from(x_index) * SQUARE_SIZE,
                            f32::from(y_index) * SQUARE_SIZE,
                        )
                    })
                    .collect::<Vec<(f32, f32)>>()
            })
            .collect();

        MainState {
            squares,
            current_block,
            positions,
            update_timer: 0,
            held_block: None,
            queue: generate_queue(),
            block_index: 0,
            used_hold: false,
            queued_queue: generate_queue(),
            lines: 0,
        }
    }

    fn try_translate(&mut self, x: i8, y: i8) {
        let translated = self.current_block.translate(x, y);
        if translated.is_valid(&self.squares) {
            self.current_block = translated;
        }
    }

    fn update_queue(&mut self) {
        let mut rng = thread_rng();
        self.queue = self.queued_queue;
        self.queued_queue.shuffle(&mut rng);
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update_timer += 1;
        if self.update_timer >= TICK_INTERVAL {
            self.update_timer = 0;

            let translated = self.current_block.translate(0, 1);

            if translated.is_valid(&self.squares) {
                self.current_block = translated;
            } else {
                if self.block_index == 14 {
                    self.update_queue();
                    self.block_index = 0;
                }

                let blocktype = TYPES[self.queue[self.block_index]];
                self.block_index += 1;

                if self
                    .current_block
                    .squares
                    .iter()
                    .any(|square| square.pos.1 < 0)
                {
                    println!("Game Over");
                    println!(
                        "You completed {} lines in {}",
                        self.lines,
                        duration_display(timer::time_since_start(ctx))
                    );
                    ggez::quit(ctx);
                };

                self.squares.append(&mut self.current_block.squares);
                self.used_hold = false;

                let (min_y, max_y) = self.current_block.squares.iter().fold(
                    (0, Y_SQUARES),
                    |(min, max), current| {
                        let current_y = current.pos.1;
                        if current_y < min {
                            (current_y, max)
                        } else if current_y > max {
                            (min, current_y)
                        } else {
                            (min, max)
                        }
                    },
                );

                (min_y..=max_y).for_each(|y| {
                    let row = self.squares.iter().filter(|square| square.pos.1 == y);
                    if row.count() >= X_SQUARES.try_into().unwrap() {
                        self.lines += 1;

                        self.squares = self
                            .squares
                            .iter()
                            .filter(|square| square.pos.1 != y)
                            .map(|square| {
                                if square.pos.1 < y {
                                    square.translate(0, 1)
                                } else {
                                    *square
                                }
                            })
                            .collect()
                    }
                });

                self.current_block = Block::new(blocktype, Orientation::Up)
                    .translate(X_SQUARES / 2, 0)
                    .translate(0, -5);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let info_text = graphics::Text::new(
            format! {"Lines: {}\n{}", self.lines, duration_display(timer::time_since_start(ctx))},
        );

        graphics::draw(
            ctx,
            &info_text,
            DrawParam::new()
                .dest([SCREEN_WIDTH + 25., 10.])
                .scale([1.3, 1.3]),
        )
        .unwrap();

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

        //#[rustfmt::skip]
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
            let held = Block::new(held_type, Orientation::Up).translate(X_SQUARES + 2, 2);

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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.try_translate(-1, 0),
            KeyCode::Right => self.try_translate(1, 0),
            KeyCode::Down => self.try_translate(0, 1),
            KeyCode::Up => {
                let rotated = self.current_block.rotate();

                let overflow =
                    self.current_block
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
            KeyCode::Space => {
                self.try_translate(0, self.current_block.max_drop(&self.squares));
                self.update_timer = TICK_INTERVAL;
            }
            KeyCode::C => {
                if !self.used_hold {
                    self.used_hold = true;
                    let saved_current = self.current_block.blocktype;
                    self.current_block = match self.held_block {
                        Some(blocktype) => {
                            Block::new(blocktype, Orientation::Up).translate(X_SQUARES / 2, -5)
                        }
                        None => {
                            //duplicated code oops
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
            _ => {}
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Tetrs", "Fish")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetrs"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_WIDTHER, SCREEN_HEIGHT)
                .resizable(false),
        )
        .build()
        .expect("error building context");

    let main_state = &mut MainState::new();

    // let resource_dir = ggez::filesystem::resources_dir(ctx);

    // let font = ggez::filesystem::open(ctx, resource_dir.join("Nitaka.ttf"));
    // dbg!(font);

    event::run(ctx, event_loop, main_state)
}

fn duration_display(duration: std::time::Duration) -> String {
    let (mins, secs) = (duration.as_secs() / 60, duration.as_secs() % 60);
    format!("{}:{}", mins, secs)
}
