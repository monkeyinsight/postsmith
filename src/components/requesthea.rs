use std::cell::RefCell;
use std::vec;

use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
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
    pub headers: Vec<RequestHeader>,
    pub selected_header: usize,
    pub writable: bool,
    pub is_modal_open: bool,
    pub selected_body_tab: usize,
    pub body_tabs: Vec<RequestHeaders>,
    pub editing_header: Option<EditingField>,
    pub show_selection: bool,
    pub show_body: bool,
}

pub struct RequestHeader {
    pub title: String,
    pub value: String,
    pub previous_value: String,
}

pub enum EditingField {
    Title,
    Value,
    PreviousValue,
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
            headers: vec![
                RequestHeader {
                    title: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                    previous_value: "".to_string(),
                },
                RequestHeader {
                    title: "Authorization".to_string(),
                    value: "Bearer token".to_string(),
                    previous_value: "".to_string(),
                },
            ],
            selected_header: 0,
            writable: false,
            is_modal_open: false,
            selected_body_tab: 0,
            body_tabs: RequestHeaders::all_request(),
            editing_header: None,
            show_selection: false,
            show_body: false,
        }
    }

    fn save_header(&mut self) {
        if let Some(editing_field) = self.editing_header.take() {
            match editing_field {
                EditingField::Title => {
                    self.headers[self.selected_header].title = self.input.value().to_string();
                }
                EditingField::Value => {
                    self.headers[self.selected_header].value = self.input.value().to_string();
                }
                EditingField::PreviousValue => {
                    self.headers[self.selected_header].previous_value = self.input.value().to_string();
                }
            }
            self.input = Input::default(); 
        }
    }

    fn save_body(&mut self) {
       
    }
}

impl Component for RequestComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([ratatui::layout::Constraint::Length(3), ratatui::layout::Constraint::Min(0)].as_ref())
            .split(area);

        if self.show_body {
            let body_tab_spans: Vec<Span> = self.body_tabs.iter().enumerate().map(|(i, tab)| {
                let tab_text = format!("{} ", tab.to_string()); 
                if i == self.selected_body_tab {
                    Span::styled(tab_text, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
                } else {
                    Span::raw(tab_text)
                }
            }).collect();

            let body_tab_line = Line::from(body_tab_spans);

            let body_tabs_paragraph = Paragraph::new(Text::from(vec![body_tab_line]))
                .block(Block::new()
                    .borders(Borders::ALL)
                    .title("Body Tabs")
                    .style(Style::default().fg(if is_active {
                        Color::Green
                    } else {
                        Color::White
                    })));

            f.render_widget(body_tabs_paragraph, chunks[0]);

            let input_block = Block::new()
                .borders(Borders::ALL)
                .title("Body Input")
                .style(Style::default().fg(if is_active {
                    Color::Green
                } else {
                    Color::White
                }));

            let paragraph = Paragraph::new(self.input.value())
                .block(input_block)
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            if is_active && self.writable {
                f.set_cursor(
                    chunks[1].x + self.input.visual_cursor() as u16 + 1,
                    chunks[1].y + 1,
                );
            }

            f.render_widget(paragraph, chunks[1]);
        } else {
            let header_spans: Vec<Span> = self.headers.iter().enumerate().map(|(i, header)| {
                let header_text = format!("{}: {} (prev: {}) ", header.title, header.value, header.previous_value); 
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

            if self.show_selection {
                let size = f.size();
                let menu_width = 30;
                let menu_height = 10;
                let menu_x = (size.width - menu_width) / 2;
                let menu_y = (size.height - menu_height) / 2;
                let area = Rect::new(menu_x, menu_y, menu_width, menu_height);

                let selection_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Select Field")
                    .style(Style::default().fg(Color::Green));

                let fields = vec!["Title", "Value", "Previous Value"];
                let field_list: Vec<ListItem> = fields
                    .iter()
                    .map(|field| ListItem::new(*field))
                    .collect();

                let list = List::new(field_list)
                    .block(selection_block)
                    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .highlight_symbol(">>");

                f.render_stateful_widget(list, area, &mut self.list_state.borrow_mut());
            }

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

            if is_active && self.writable {
                f.set_cursor(
                    chunks[1].x + self.input.visual_cursor() as u16 + 1,
                    chunks[1].y + 1,
                );
            }

            f.render_widget(paragraph, chunks[1]);
        }
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('[') => {
                if !self.writable {
                    if self.show_body {
                        self.show_body = false;
                    }
                }
            }
            KeyCode::Char(']') => {
                if !self.writable {
                    if !self.show_body {
                        self.show_body = true;
                    }
                }
            }
            KeyCode::Left => {
                if self.show_body {
                    if self.selected_body_tab > 0 {
                        self.selected_body_tab -= 1;
                    } else {
                        self.selected_body_tab = self.body_tabs.len() - 1;
                    }
                } else if !self.writable {
                    if self.selected_header > 0 {
                        self.selected_header -= 1;
                    } else {
                        self.selected_header = self.headers.len() - 1;
                    }
                }
            }
            KeyCode::Right => {
                if self.show_body {
                    if self.selected_body_tab < self.body_tabs.len() - 1 {
                        self.selected_body_tab += 1;
                    } else {
                        self.selected_body_tab = 0;
                    }
                } else if !self.writable {
                    if self.selected_header < self.headers.len() - 1 {
                        self.selected_header += 1;
                    } else {
                        self.selected_header = 0;
                    }
                }
            }
            KeyCode::Enter => {
                if self.show_selection {
                    if let Some(selected) = self.list_state.borrow().selected() {
                        match selected {
                            0 => self.editing_header = Some(EditingField::Title),
                            1 => self.editing_header = Some(EditingField::Value),
                            2 => self.editing_header = Some(EditingField::PreviousValue),
                            _ => {}
                        }
                        self.input = Input::default();
                        match self.editing_header {
                            Some(EditingField::Title) => {
                                self.input = Input::from(self.headers[self.selected_header].title.clone());
                            }
                            Some(EditingField::Value) => {
                                self.input = Input::from(self.headers[self.selected_header].value.clone());
                            }
                            Some(EditingField::PreviousValue) => {
                                self.input = Input::from(self.headers[self.selected_header].previous_value.clone());
                            }
                            None => {}
                        }
                        self.show_selection = false;
                        self.writable = true;
                    }
                } else if self.writable {
                    if self.show_body {
                        self.save_body();
                    } else {
                        self.save_header();
                    }
                    self.writable = false;
                } else {
                    self.show_selection = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.show_selection {
                    let i = match self.list_state.borrow().selected() {
                        Some(i) => {
                            if i == 0 {
                                2
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.list_state.borrow_mut().select(Some(i));
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                if self.show_selection {
                    let i = match self.list_state.borrow().selected() {
                        Some(i) => {
                            if i == 2 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.list_state.borrow_mut().select(Some(i));
                }
            }
            KeyCode::Esc => {
                self.show_selection = false;
                self.writable = false;
            }
            _ => {
                if self.writable {
                    self.input.handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                }
            }
        }
    }
}
