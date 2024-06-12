use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;

use crossterm::event::KeyCode;

use crate::ui::Component;

pub struct InputComponent {
    pub value: String,
}

impl InputComponent {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl Component for InputComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Input")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));
        
        let value_clone = self.value.to_string();
        let paragraph = Paragraph::new(value_clone)
            .block(block)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(paragraph, area);
    }

    fn keybinds(&mut self, _key: KeyCode) {

    }
}
