use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color,  Style};
use ratatui::Frame;
use crossterm::event::KeyCode;
use ratatui::text::{ Span, Text};

use crate::ui::Component;
use ratatui::{prelude::*, widgets::*};

pub struct HistoryComponent {
    pub history: String,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl HistoryComponent {
    /*pub fn new() -> Self {
        Self {
            history: String::new(),
            scroll_x: 0,
            scroll_y: 0,
        }
    }*/
    pub fn new_with_history(history: String) -> Self {
        Self {
            history: history,
            scroll_x: 0,
            scroll_y: 0,
        }
    }
}

impl Component for HistoryComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("History")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));

            let lines: Vec<Line> = self.history
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let line_style = if i as u16 == self.scroll_y {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default()
                };
                Line::from(Span::styled(line, line_style))
            })
            .collect();

        let paragraph = Paragraph::new(Text::from(lines))
            .block(block)
            .style(Style::default().fg(Color::Green))
            .scroll((self.scroll_y, self.scroll_x));

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(self.history.lines().count())
            .position(self.scroll_y as usize);

        f.render_widget(paragraph, area);

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
        let max_scroll_y = self.history.lines().count().saturating_sub(1) as u16;
        match key {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.scroll_y > 0 {
                    self.scroll_y -= 1;
                } else {
                    self.scroll_y = max_scroll_y;
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J')=> {
                if self.scroll_y < max_scroll_y {
                    self.scroll_y += 1;
                } else {
                    self.scroll_y = 0;
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
            _ => {}
        }
    }
}