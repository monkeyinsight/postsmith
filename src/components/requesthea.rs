use std::cell::RefCell;
use std::vec;

use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, ListState, Paragraph};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;

use ratatui::widgets::{List, ListItem};

use crossterm::event::{KeyCode, KeyEvent, Event};

use crate::ui::Component;

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

pub struct RequestComponent {
    pub input: Input,
    pub list_state: RefCell<ListState>,
}

pub enum RequestHeaders {
    none, formdata, xwwwformundeclored, raw, binary, graphql
}

impl  RequestHeaders {
    pub fn all_request() -> Vec<RequestHeaders>{
       vec![
           RequestHeaders::none,
           RequestHeaders::formdata,
           RequestHeaders::xwwwformundeclored,
           RequestHeaders::raw,
           RequestHeaders::binary,
           RequestHeaders::graphql,
       ]
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            RequestHeaders::none => "none",
            RequestHeaders::formdata => "formdata",
            RequestHeaders::xwwwformundeclored => "x-www-form-urlencoded",
            RequestHeaders::raw => "raw",
            RequestHeaders::binary => "binary",
            RequestHeaders::graphql => "graphql",
        }
    }
}

impl RequestComponent {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
            list_state: RefCell::new(ListState::default()),
        }
    }
}

impl Component for RequestComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let block = Block::new()
            .borders(Borders::ALL)
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));

        let paragraph = Paragraph::new(self.input.value())
            .block(block)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        if is_active {
            f.set_cursor(
                area.x + self.input.visual_cursor() as u16 + 1,
                area.y + 1,
            );
        }

        f.render_widget(paragraph, area);
    }

    fn keybinds(&mut self, key: KeyCode) {
        self.input.handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
    }
}