use ggez::{event, graphics::Font, GameResult};

mod game_state;
mod main_state;
mod menu_state;
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

    let font = Font::new(ctx, "/fonts/Xolonium-Regular.ttf").unwrap();
    let main_state = &mut MainState {
        current_state: Box::new(menu_state::MenuState::new(font, None)),
        font,
    };

    event::run(ctx, event_loop, main_state)
}
