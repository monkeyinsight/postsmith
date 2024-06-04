use std::cell::RefCell;
use std::vec;

use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, ListState, Paragraph};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;
use ratatui::text::{Line, Span, Text};

use crossterm::event::{KeyCode, KeyEvent, Event};

use crate::ui::Component;

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

pub struct RequestComponent {
    pub input: Input,
    pub list_state: RefCell<ListState>,
    pub headers: Vec<RequestHeaders>,
    pub selected_header: usize,
}

pub enum RequestHeaders {
    None,
    FormData,
    Xwwwformundeclored,
    Raw,
    Binary,
    Graphql,
}

impl RequestHeaders {
    pub fn all_request() -> Vec<RequestHeaders> {
        vec![
            RequestHeaders::None,
            RequestHeaders::FormData,
            RequestHeaders::Xwwwformundeclored,
            RequestHeaders::Raw,
            RequestHeaders::Binary,
            RequestHeaders::Graphql,
        ]
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            RequestHeaders::None => "none",
            RequestHeaders::FormData => "form-data",
            RequestHeaders::Xwwwformundeclored => "x-www-form-urlencoded",
            RequestHeaders::Raw => "raw",
            RequestHeaders::Binary => "binary",
            RequestHeaders::Graphql => "GraphQL",
        }
    }
}

impl RequestComponent {
    pub fn new() -> Self {
        let headers = RequestHeaders::all_request();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            input: Input::default(),
            list_state: RefCell::new(list_state),
            headers,
            selected_header: 0,
        }
    }
}

impl Component for RequestComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([ratatui::layout::Constraint::Length(3), ratatui::layout::Constraint::Min(0)].as_ref())
            .split(area);

        let header_spans: Vec<Span> = self.headers.iter().enumerate().map(|(i, header)| {
            let header_text = header.to_string();
            if i == self.selected_header {
                Span::styled(header_text, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            } else {
                Span::raw(header_text)
            }
        }).collect();

        let header_line = Line::from(header_spans);

        let headers_paragraph = Paragraph::new(Text::from(vec![header_line]))
            .block(Block::new()
                .borders(Borders::ALL)
                .title("Request Headers")
                .style(Style::default().fg(if is_active {
                    Color::Green
                } else {
                    Color::White
                })));

        f.render_widget(headers_paragraph, chunks[0]);

        let input_block = Block::new()
            .borders(Borders::ALL)
            .title("Input")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));

        let paragraph = Paragraph::new(self.input.value())
            .block(input_block)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        if is_active {
            f.set_cursor(
                chunks[1].x + self.input.visual_cursor() as u16 + 1,
                chunks[1].y + 1,
            );
        }

        f.render_widget(paragraph, chunks[1]);
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left | KeyCode::Char('[') => {
                if self.selected_header > 0 {
                    self.selected_header -= 1;
                } else{
                    self.selected_header = self.headers.len() - 1
                }
            }
            KeyCode::Right | KeyCode::Char(']') => {
                if self.selected_header < self.headers.len() - 1 {
                    self.selected_header += 1;
                } else {
                    self.selected_header = 0;
                }
            }
            _ => {
                self.input.handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
            }
        }
    }
}
