use dirs::config_dir;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Style};
use ratatui::Frame;
use crossterm::event::KeyCode;
use ratatui::text::{Span, Text};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

use crate::ui::Component;
use ratatui::prelude::*;

use super::requesthea::{RequestHeader, RequestHeaders};
use super::selector::HttpMethod;


#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    date: String,
    action: String,
    url: String,
    body_content: String,
    headers: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    history: Vec<History>,
}

impl Session {
    pub fn new() -> Self {
        let mut session = Self {
            history: Vec::new(),
        };
        session.load().unwrap_or_default();
        session
    }

    pub fn push_history(&mut self, request: &str, url: String, body_content: Vec<(RequestHeaders, String)>, headers: Vec<RequestHeader>) {
        let body_content_str = body_content.iter()
            .map(|(key, value)| format!("{:?}: {}", key, value))
            .collect::<Vec<_>>()
            .join(", ");

        let headers_str = headers.iter()
            .map(|header| format!("{}: {}", header.key, header.value))
            .collect::<Vec<_>>()
            .join(", ");

        let history = History {
            date: chrono::offset::Local::now().to_string(),
            action: request.to_string(),
            url,
            headers: headers_str,
            body_content: body_content_str,
        };

        self.history.push(history);

        self.save().unwrap();
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = config_dir().unwrap().join("postsmith");
        if !config_path.exists() {
            fs::create_dir_all(&config_path)?; // Create directory if it doesn't exist
        }
        let file_path = config_path.join("session");
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Overwrite existing file
            .open(file_path)?;

        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())?;
        Ok(())
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = config_dir().unwrap().join("postsmith/session");
        if file_path.exists() {
            let data = fs::read_to_string(file_path)?;
            let session: Session = serde_json::from_str(&data)?;
            self.history = session.history;
        }
        Ok(())
    }

    pub fn get_history(&self) -> String {
        self.history
            .iter()
            .rev()
            .map(|h| format!("{} - {} - {} - {}  - {}\n", h.date, h.action, h.url, h.body_content, h.headers))
            .collect()
    }

    pub fn get_history_entries(&self) -> Vec<&History> {
        self.history.iter().rev().collect()
    }

    pub fn get_current_url(&self, scroll_y: usize) -> Option<String> {
        let entries = self.get_history_entries();
        if scroll_y >= entries.len() {
            None
        } else {
            Some(entries[scroll_y].url.clone())
        }
    }

    pub fn get_body_content(&self, scroll_y: usize) -> Option<Vec<(RequestHeaders, String)>> {
        let entries = self.get_history_entries();
        if scroll_y >= entries.len() {
            None
        } else {
            let body_content_str = &entries[scroll_y].body_content;
            Some(
                body_content_str.split(", ")
                    .filter_map(|s| {
                        let parts: Vec<&str> = s.split(": ").collect();
                        if parts.len() == 2 {
                            if let Some(header) = RequestHeaders::from_str(parts[0]) {
                                Some((header, parts[1].to_string()))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            )
        }
    }

    // Function to get headers as a vector
    pub fn get_headers(&self, scroll_y: usize) -> Option<Vec<RequestHeader>> {
        let entries = self.get_history_entries();
        if scroll_y >= entries.len() {
            None
        } else {
            let headers_str = &entries[scroll_y].headers;
            Some(
                headers_str.split(", ")
                    .filter_map(|s| {
                        let parts: Vec<&str> = s.split(": ").collect();
                        if parts.len() == 2 {
                            Some(RequestHeader {
                                key: parts[0].to_string(),
                                value: parts[1].to_string(),
                                previous_value: String::new(), // Assuming you don't store previous_value in History
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            )
        }
    }

    pub fn get_method(&self, scroll_y: usize) -> Option<HttpMethod> {
        let entries = self.get_history_entries();
        if scroll_y >= entries.len() {
            None
        } else {
            HttpMethod::from_str(&entries[scroll_y].action)
        }

    }
}

pub struct HistoryComponent {
    pub history: String,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl HistoryComponent {
    pub fn new_with_history(history: String) -> Self {
        Self {
            history,
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

        f.render_widget(paragraph, area);
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
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
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
            KeyCode::Enter => {
             /*    let session = Session::new();
                if let Some(url) = session.get_current_url(self.scroll_y as usize) {
                 //   println!("Selected URL: {}", url);
                  app_state.input_component.value = url;
                } */
            }
            _ => {}
        }
    }
}
