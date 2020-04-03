use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawMode, DrawParam, Font, Rect, Scale, Text, TextFragment},
    Context, GameResult,
};

use crate::main_state::{Signal, SignalState, StateTrait};
use std::time::Duration;

pub struct Button {
    rect: Rect,
    hovered: bool,
    color: Color,
    hover_color: Color,
    text: graphics::Text,
    signal: Signal,
}

impl Button {
    fn new(
        text: &str,
        font: Font,
        color: Color,
        hover_color: Color,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        signal: Signal,
    ) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            hovered: false,
            color,
            hover_color,
            text: Text::new(
                TextFragment::new(text)
                    .scale(Scale::uniform(0.75 * height))
                    .font(font),
            ),
            signal,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameOverData {
    pub lines: usize,
    pub time: Duration,
}

pub struct MenuState {
    buttons: Vec<Button>,
    header_text: Text,
    game_over_text: Option<Text>,
    sent_signals: Vec<Signal>,
}

impl MenuState {
    pub fn new(text_font: Font, game_over_data: Option<GameOverData>) -> Self {
        let header_text = Text::new(
            TextFragment::new("TETRS")
                .scale(Scale::uniform(120.0))
                .font(text_font),
        );
        let play_button = Button::new(
            "PLAY",
            text_font,
            Color::new(1.0, 0.0, 0.0, 1.0),
            Color::new(0.8, 0.0, 0.0, 1.0),
            117.5,
            250.0,
            275.0,
            100.0,
            Signal::StartGame,
        );

        let game_over_text = game_over_data.map(|data| {
            Text::new(
                TextFragment::new(format!(
                    "You Lose! \n Lines: {} \n Time: {}s",
                    data.lines,
                    data.time.as_secs()
                ))
                .font(text_font)
                .scale(Scale::uniform(49.0)),
            )
        });

        MenuState {
            header_text,
            buttons: vec![play_button],
            game_over_text,
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

        graphics::draw(ctx, &self.header_text, DrawParam::new().dest([62.5, 50.0]))?;

        self.buttons.iter().for_each(|btn| {
            draw_button(&btn, ctx).unwrap();
        });

        if let Some(text) = &self.game_over_text {
            graphics::draw(ctx, text, DrawParam::new().dest([150.0, 400.0]))?;
        }

        graphics::present(ctx).expect("error rendering");
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let ggez::input::mouse::MouseButton::Left = button {
            let mouse_rect = Rect::new(x, y, 1.0, 1.0);

            self.sent_signals = self
                .buttons
                .iter()
                .filter_map(|btn| {
                    if btn.rect.overlaps(&mouse_rect) {
                        Some(btn.signal)
                    } else {
                        None
                    }
                })
                .collect();
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
    graphics::draw(
        ctx,
        &button.text,
        DrawParam::new().dest([
            button.rect.x + (button.rect.w * 0.1825),
            button.rect.y + (button.rect.h * 0.125),
        ]),
    )
    .unwrap();
    Ok(())
}
