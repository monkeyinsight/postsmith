use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::Frame;
use crossterm::event::KeyCode;

use crate::ui::Component;

pub struct InputComponent {
    pub input: String,
    pub scroll_x: u16,
}

impl InputComponent {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            scroll_x: 0,
        }
    }
}

impl Component for InputComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_active: bool) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));
        let paragraph = Paragraph::new(self.input.clone())
            .block(block)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .scroll((0, self.scroll_x));
        f.render_widget(paragraph, area);
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Left => {
                if self.scroll_x > 0 {
                    self.scroll_x -= 1;
                }
            }
            KeyCode::Right => {
                self.scroll_x += 1;
            }
            _ => {}
        }
    }
}
