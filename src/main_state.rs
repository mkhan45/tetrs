use ggez::{
    event::EventHandler,
    Context, GameResult,
};

use crate::game_state;

#[derive(Clone, Copy, Debug)]
pub enum Signal {
    StartGame,
}

pub trait SignalState {
    fn signals(&mut self) -> &mut Vec<Signal>;
}

pub trait StateTrait: SignalState + EventHandler {}

pub struct MainState {
    pub current_state: Box<dyn StateTrait>,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.update(ctx)?;

        let mut signals = self.current_state.signals().clone();
        signals.drain(..).for_each(|signal|{
            self.process_signal(signal);
        });
        self.current_state.signals().clear();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: ggez::input::mouse::MouseButton, x: f32, y: f32) {
        self.current_state.mouse_button_down_event(ctx, button, x, y);
    }
}

impl MainState {
    fn process_signal(&mut self, signal: Signal) {
        match signal {
            Signal::StartGame => {
                self.current_state = Box::new(game_state::GameState::new());
            }
        }
    }
}
