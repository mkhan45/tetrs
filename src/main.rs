use ggez::{event, GameResult};

mod game_state;
mod main_state;

mod block;

mod input;

mod actions;

mod consts;
use consts::*;

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

    let main_state = &mut game_state::GameState::new();

    event::run(ctx, event_loop, main_state)
}

