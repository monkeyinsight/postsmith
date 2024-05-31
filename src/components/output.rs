use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::style::{Color, Style};
use tui::Frame;
use crossterm::event::KeyCode;

use crate::ui::Component;

pub struct OutputComponent {
    pub message: String,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl OutputComponent {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            scroll_x: 0,
            scroll_y: 0,
        }
    }
}

impl Component for OutputComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_active: bool) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Message")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));
        let paragraph = Paragraph::new(self.message.clone())
            .block(block)
            .style(Style::default().fg(Color::Green))
            .scroll((self.scroll_y, self.scroll_x));
        f.render_widget(paragraph, area);
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.scroll_y > 0 {
                    self.scroll_y -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J')=> {
                self.scroll_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                if self.scroll_x > 0 {
                    self.scroll_x -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                self.scroll_x += 1;
            }
            _ => {}
        }
    }
}
