use ggez::{
    event::EventHandler,
    Context, GameResult,
};

pub struct MainState {
    current_state: Box<dyn EventHandler>,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.draw(ctx)
    }
}
