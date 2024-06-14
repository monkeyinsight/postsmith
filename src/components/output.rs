use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::style::{Color, Style};
use ratatui::Frame;
use crossterm::event::KeyCode;
use std::fs::File;
use std::io::Write;
use std::process::Command;

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

    fn save_message_to_file(&self) -> std::io::Result<String> {
        let file_path = "/tmp/output.json";
        let mut file = File::create(file_path)?;
        file.write_all(self.message.as_bytes())?;
        Ok(file_path.to_string())
    }

    fn open_in_editor(&self) -> std::io::Result<()> {
        let file_path = self.save_message_to_file()?;
        Command::new(std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()))
            .arg(&file_path)
            .status()?;
        Ok(())
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
            .style(Style::default().fg(Color::White))
            .scroll((self.scroll_y, 0));
        
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        
        let mut scrollbar_state = ScrollbarState::new(self.message.lines().count())
            .position(self.scroll_y as usize);
        
        f.render_widget(paragraph, area);
        f.render_stateful_widget(
            scrollbar,
            area.inner(&ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }

    fn keybinds(&mut self, key: KeyCode) {
        let max_scroll_y = self.message.lines().count().saturating_sub(1) as u16;
        match key {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.scroll_y > 0 {
                    self.scroll_y -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                if self.scroll_y < max_scroll_y {
                    self.scroll_y += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                if self.scroll_x > 0 {
                    self.scroll_x -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                self.scroll_x += 1;
            }
            KeyCode::Char('e') => {
                if let Err(e) = self.open_in_editor() {
                    eprintln!("Failed to open editor: {}", e);
                }
            }
            _ => {}
        }
    }
}
