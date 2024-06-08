use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color,  Style};
use ratatui::Frame;
use crossterm::event::KeyCode;
use std::{
    env::{temp_dir, var},
    fs::File,
    process::Command,
};
use std::io::Write as _;

use crate::ui::Component;
use ratatui::{prelude::*, widgets::*};

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
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
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
            .scroll((self.scroll_y, 0));

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(self.message.lines().count())
            .position(self.scroll_y as usize);

        f.render_widget(paragraph, area);

        // Render the scrollbar
        f.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.scroll_y > 0 {
                    self.scroll_y -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.scroll_x > 0 {
                    self.scroll_x -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.scroll_x += 1;
            }
            KeyCode::Char('e') => {
                let editor = var("EDITOR").unwrap();
                let mut file_path = temp_dir();
                file_path.push("edit");
                let mut file = File::create(&file_path).expect("Could not create file");
                let _ = file.write_all(self.message.clone().as_bytes());

                Command::new(editor)
                    .arg(&file_path)
                    .status()
                    .expect("Cannot launch editor");
            }
            _ => {}
        }
    }
}
