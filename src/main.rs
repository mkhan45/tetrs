use ggez::{event, GameResult};

mod game_state;
mod menu_state;
mod main_state;
use main_state::MainState;

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

    let game_state = game_state::GameState::new();
    let menu_state = menu_state::MenuState::default();
    let main_state = &mut MainState {
        current_state: Box::new(menu_state)
    };

    event::run(ctx, event_loop, main_state)
}

