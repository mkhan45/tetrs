extern crate ggez;
use ggez::{
    event, nalgebra as na, GameResult, Context, graphics,
    graphics::{DrawMode, Color, Mesh, Rect, MeshBuilder, DrawParam},
    event::{EventHandler},
    input::{keyboard}, input::keyboard::KeyCode,
};

use std::convert::TryInto;

extern crate rand;
use rand::{Rng, thread_rng};

mod block;
use block::*;

type Vector = na::Vector2<f32>;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 300.;

const X_SQUARES: u16 = 15;
const Y_SQUARES: u16 = 30;

const SQUARE_SIZE: f32 = SCREEN_HEIGHT / Y_SQUARES as f32;

const TICK_INTERVAL: usize = 10;

#[derive(Clone)]
struct MainState{
    pub squares: Vec<Square>,
    pub current_block: Block,
}

impl MainState {
    fn new() -> Self {
        let squares = Vec::with_capacity((X_SQUARES * Y_SQUARES).try_into().unwrap());

        let current_block = Block::new(BlockType::Line, Orientation::Up).translate(X_SQUARES as i16/2, 0);

        MainState{squares, current_block}
    }

    fn reset(&mut self) {
    }

    fn try_translate(&mut self, x: i16, y: i16){
        if self.current_block.translate(x, y).is_valid(&self.squares){
            self.current_block = self.current_block.translate(x, y) 
        }
    }
}

impl EventHandler for MainState{
    fn update(&mut self, ctx: &mut Context) -> GameResult{
        if ggez::timer::ticks(ctx) % TICK_INTERVAL == 0{
            let should_translate = !self.current_block.squares.iter().any(|&current_square|{
                let mut new_square = current_square.rect.clone();
                new_square.translate([0., SQUARE_SIZE]);
                new_square.y + 0.5 >= SCREEN_HEIGHT || 
                    self.squares.iter().any(|&board_square| new_square.overlaps(&board_square.rect))
            });

            if should_translate { 
                self.current_block.squares.iter_mut().for_each(|square|{
                    square.rect.translate([0., SQUARE_SIZE]);
                });
            } else {
                let types = [BlockType::Line, BlockType::Square];
                let blocktype = types[(rand::random::<f32>() * types.len() as f32) as usize];

                self.squares.append(&mut self.current_block.squares);
                self.current_block = Block::new(blocktype, Orientation::Up);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut mesh = MeshBuilder::new();

        self.current_block.squares.iter().for_each(|square|{
            mesh.rectangle(DrawMode::fill(), square.rect, square.color);
        });

        self.squares.iter().for_each(|square|{
            mesh.rectangle(DrawMode::fill(), square.rect, square.color);
        });

        let preview = self.current_block.translate(0, self.current_block.max_drop(&self.squares));
        preview.squares.iter().for_each(|square|{
            mesh.rectangle(DrawMode::fill(), square.rect, Color::new(1.0, 1.0, 1.0, 0.5));
        });
        

        let mesh = &mesh.build(ctx).unwrap();

        graphics::draw(ctx, mesh, DrawParam::new()).unwrap();
        graphics::present(ctx).expect("error rendering");

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: keyboard::KeyCode, _keymods: keyboard::KeyMods, _repeat: bool){
        match keycode{
            KeyCode::Left => self.try_translate(-1, 0),
            KeyCode::Right => self.try_translate(1, 0),
            KeyCode::Down => self.try_translate(0, 2),
            KeyCode::Space => {
                self.try_translate(0, self.current_block.max_drop(&self.squares));
                self.try_translate(0, 1); //this is the smallest brain thing i've ever done
            }
            _ => {},
        }
    }

}

fn main() -> GameResult{
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Fish")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build().expect("error building context");

    let main_state = &mut MainState::new();

    event::run(ctx, event_loop, main_state)
}
