use ggez::{
    event::EventHandler,
    Context, GameResult,
    graphics::{self, Color, Rect, DrawMode, DrawParam, Text, TextFragment, Scale},
};

use crate::main_state::{Signal, SignalState, StateTrait};

pub struct Button {
    rect: Rect,
    hovered: bool,
    color: Color,
    hover_color: Color,
    text: graphics::Text,
    signal: Signal,
}

impl Button {
    fn new(text: &str, color: Color, hover_color: Color, x: f32, y: f32, width: f32, height: f32, signal: Signal) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            hovered: false,
            color,
            hover_color,
            text: Text::new(TextFragment::new(text).scale(Scale::uniform(0.25 * height))),
            signal,
        }
    }
}

pub struct MenuState {
    buttons: Vec<Button>,
    sent_signals: Vec<Signal>,
}

impl Default for MenuState {
    fn default() -> Self {
        let play_button = Button::new("Play", Color::new(1.0, 0.0, 0.0, 1.0), Color::new(0.8, 0.0, 0.0, 1.0), 200.0, 200.0, 150.0, 100.0, Signal::StartGame);
        MenuState{
            buttons: vec![play_button],
            sent_signals: Vec::new(),
        }
    }
}

impl EventHandler for MenuState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_rect = {
            let point = ggez::input::mouse::position(ctx);
            Rect::new(point.x, point.y, 1.0, 1.0)
        };
        
        self.buttons.iter_mut().for_each(|btn| {
            btn.hovered = btn.rect.overlaps(&mouse_rect);
        });

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        self.buttons.iter().for_each(|btn|{
            draw_button(&btn, ctx).unwrap();
        });

        graphics::present(ctx).expect("error rendering");
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: ggez::input::mouse::MouseButton, x: f32, y: f32) {
        dbg!(button);
        if let ggez::input::mouse::MouseButton::Left = button {
            let mouse_rect = Rect::new(x, y, 1.0, 1.0);

            self.sent_signals = self.buttons.iter().filter_map(|btn| {
                if btn.rect.overlaps(&mouse_rect) {
                    Some(btn.signal)
                } else {
                    None
                }
            }).collect();
        }
    }
}

impl SignalState for MenuState {
    fn signals(&mut self) -> &mut Vec<Signal> {
        &mut self.sent_signals
    }
}

impl StateTrait for MenuState {}

fn draw_button(button: &Button, ctx: &mut Context) -> GameResult {
    let color = if button.hovered {
        button.hover_color
    } else {
        button.color
    };

    let rect = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), button.rect, color).unwrap();
    graphics::draw(ctx, &rect, DrawParam::new()).unwrap();

    graphics::draw(ctx, &button.text, DrawParam::new().dest([button.rect.x, button.rect.y])).unwrap();
    Ok(())
}
