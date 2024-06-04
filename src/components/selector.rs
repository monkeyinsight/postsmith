use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;
use std::cell::RefCell;
use ratatui::widgets::{List, ListItem};

use crate::ui::Component;



#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
}

impl HttpMethod {
    pub fn all_methods() -> Vec<HttpMethod> {
        vec![
            HttpMethod::GET,
            HttpMethod::POST,
            HttpMethod::PUT,
            HttpMethod::DELETE,
            HttpMethod::PATCH,
            HttpMethod::HEAD,
            HttpMethod::OPTIONS,
        ]
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

pub struct SelectorComponent {
    pub method: HttpMethod,
    pub show_selection: bool,
    pub list_state: RefCell<ListState>,
}

impl SelectorComponent {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            method: HttpMethod::GET,
            show_selection: false,
            list_state: RefCell::new(list_state),
        }
    }
}

impl Component for SelectorComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Method")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));
        let paragraph = Paragraph::new(format!("{:?}", self.method))
            .block(block)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        f.render_widget(paragraph, area);

        if self.show_selection {
            let size = f.size();
            let menu_width = 30;
            let menu_height = 10;
            let menu_x = (size.width - menu_width) / 2;
            let menu_y = (size.height - menu_height) / 2;
            let area = Rect::new(menu_x, menu_y, menu_width, menu_height);

            let method_selection_block = Block::default()
                .borders(Borders::ALL)
                .title("Select Method")
                .style(Style::default().fg(Color::Green));
            
            let method_list: Vec<ListItem> = HttpMethod::all_methods()
                .iter()
                .map(|method| ListItem::new(method.to_string()))
                .collect();
            
            let methods = List::new(method_list)
                .block(method_selection_block)
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(methods, area, &mut self.list_state.borrow_mut());
        }
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if self.show_selection {
                    if let Some(selected) = self.list_state.borrow().selected() {
                        self.method = HttpMethod::all_methods()[selected];
                    }
                    self.show_selection = false;
                } else {
                    self.show_selection = true;
                }
            }
            KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('J')=> {
                if self.show_selection {
                    let i = match self.list_state.borrow().selected() {
                        Some(i) => {
                            if i == 0 {
                                HttpMethod::all_methods().len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.list_state.borrow_mut().select(Some(i));
                }
            }
            KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.show_selection {
                    let i = match self.list_state.borrow().selected() {
                        Some(i) => {
                            if i == HttpMethod::all_methods().len() - 1 {
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
            }
            _ => {}
        }
    }
}
