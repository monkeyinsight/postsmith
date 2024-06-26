use std::cell::RefCell;
use std::vec;

use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;
use ratatui::text::{Line, Span, Text};

use crossterm::event::{KeyCode, KeyEvent, Event};

use crate::ui::Component;

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EditingField {
    Key,
    Value,
    PreviousValue,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub struct RequestHeader {
    pub key: String,
    pub value: String,
    pub previous_value: String,
}

pub struct RequestComponent {
    pub inputs: [Input; 3],
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
    pub body_content: Vec<(RequestHeaders, String)>,
    pub adding_header: bool,
    pub delete: bool,
    pub selected_input: usize,
    pub is_editing: bool, // Flag to indicate if we are editing
    pub is_body_modal_open: bool,
}

impl RequestComponent {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            inputs: [Input::default(), Input::default(), Input::default()],
            list_state: RefCell::new(list_state),
            headers: vec![
                RequestHeader {
                    key: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                    previous_value: "".to_string(),
                },
                RequestHeader {
                    key: "Authorization".to_string(),
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
            show_body: true,
            body_content: vec![],
            adding_header: false,
            delete: false,
            selected_input: 0,
            is_editing: false, // Initialize as false
            is_body_modal_open: false,
        }
    }

    fn save_header(&mut self) {
        if self.adding_header {
            let new_header = RequestHeader {
                key: self.inputs[0].value().to_string(),
                value: self.inputs[1].value().to_string(),
                previous_value: self.inputs[2].value().to_string(),
            };
            self.headers.push(new_header);
            self.is_modal_open = false;
            self.adding_header = false;
            self.selected_input = 0;
            self.inputs = [Input::default(), Input::default(), Input::default()];
        } else if self.is_editing {
            self.headers[self.selected_header] = RequestHeader {
                key: self.inputs[0].value().to_string(),
                value: self.inputs[1].value().to_string(),
                previous_value: self.inputs[2].value().to_string(),
            };
            self.is_modal_open = false;
            self.is_editing = false;
            self.selected_input = 0;
            self.inputs = [Input::default(), Input::default(), Input::default()];
        }
    }

    fn delete_header(&mut self) {
        if !self.headers.is_empty() {
            self.headers.remove(self.selected_header);
            if self.selected_header >= self.headers.len() {
                self.selected_header = self.headers.len().saturating_sub(1);
            }
        }
    }

    fn draw_modal<B: Backend>(&self, f: &mut Frame) {
        let size = f.size();
        let modal_area_width = 80; // Adjust width as needed
        let modal_area_height = 20; // Adjust height as needed to fit the increased height of the "Key" input area
        let modal_area = Rect::new(
            (size.width - modal_area_width) / 2,
            (size.height - modal_area_height) / 2,
            modal_area_width,
            modal_area_height,
        );

        let modal_block = Block::default()
            .title(if self.is_editing { "Edit Header" } else { "Add Header" })
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        f.render_widget(modal_block, modal_area);

        let input_areas = [
            Rect::new(modal_area.x + 2, modal_area.y + 2, modal_area.width - 4, 3), // Key
            Rect::new(modal_area.x + 2, modal_area.y + 6, modal_area.width - 4, 8), // Value
            Rect::new(modal_area.x + 2, modal_area.y + 15, modal_area.width - 4, 3), // Previous Value
        ];

        for (i, input_area) in input_areas.iter().enumerate() {
            let paragraph = Paragraph::new(self.inputs[i].value())
                .block(Block::default().borders(Borders::ALL).title(match i {
                    0 => "Key",
                    1 => "Value",
                    2 => "Previous Value",
                    _ => "",
                }))
                .wrap(ratatui::widgets::Wrap { trim: false })
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            f.render_widget(paragraph, *input_area);

            if i == self.selected_input {
                let cursor_position = self.inputs[i].visual_cursor();
                let wrapped_text = self.inputs[i].value().chars().collect::<Vec<_>>();
                let mut cursor_x = input_area.x + 1;
                let mut cursor_y = input_area.y + 1;
                let mut line_length = 0;
                for (index, ch) in wrapped_text.iter().enumerate() {
                    if index == cursor_position {
                        break;
                    }
                    if *ch == '\n' || line_length >= input_area.width as usize - 2 {
                        cursor_y += 1;
                        cursor_x = input_area.x + 1;
                        line_length = 0;
                    } else {
                        cursor_x += 1;
                        line_length += 1;
                    }
                }
                f.set_cursor(cursor_x, cursor_y);
            }
        }
    }

    fn draw_body_modal<B: Backend>(&self, f: &mut Frame) {
        let size = f.size();
        let modal_area_width = 80; // Adjust width as needed
        let modal_area_height = 20; // Adjust height as needed to fit the increased height of the input area
        let modal_area = Rect::new(
            (size.width - modal_area_width) / 2,
            (size.height - modal_area_height) / 2,
            modal_area_width,
            modal_area_height,
        );

        let modal_block = Block::default()
            .title("Edit body") // Corrected title
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        f.render_widget(modal_block, modal_area);

        let input_area = Rect::new(modal_area.x + 2, modal_area.y + 2, modal_area.width - 4, modal_area.height - 4); // Single input area

        let paragraph = Paragraph::new(self.inputs[0].value())
            .block(Block::default().borders(Borders::ALL).title("Body"))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(paragraph, input_area);

        if self.selected_input == 0 {
            let cursor_position = self.inputs[0].visual_cursor();
            let wrapped_text = self.inputs[0].value().chars().collect::<Vec<_>>();
            let mut cursor_x = input_area.x + 1;
            let mut cursor_y = input_area.y + 1;
            let mut line_length = 0;
            for (index, ch) in wrapped_text.iter().enumerate() {
                if index == cursor_position {
                    break;
                }
                if *ch == '\n' || line_length >= input_area.width as usize - 2 {
                    cursor_y += 1;
                    cursor_x = input_area.x + 1;
                    line_length = 0;
                } else {
                    cursor_x += 1;
                    line_length += 1;
                }
            }
            f.set_cursor(cursor_x, cursor_y);
        }
    }

    fn load_body(&mut self) {
        let selected_tab = self.body_tabs[self.selected_body_tab].clone();
        if let Some((_, body_text)) = self.body_content.iter().find(|(tab, _)| *tab == selected_tab) {
            self.inputs[0] = Input::from(body_text.clone());
        } else {
            self.inputs[0] = Input::default();
        }
    }

    fn save_body(&mut self) {
        let selected_tab = self.body_tabs[self.selected_body_tab].clone();
        let body_text = self.inputs[0].value().to_string();

        if let Some(existing_entry) = self.body_content.iter_mut().find(|(tab, _)| *tab == selected_tab) {
            existing_entry.1 = body_text;
        } else {
            self.body_content.push((selected_tab, body_text));
        }
        self.inputs[0] = Input::default();
    }
}

impl Component for RequestComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([ratatui::layout::Constraint::Length(3), ratatui::layout::Constraint::Length(6)].as_ref())
            .split(area);

        // Always render the main UI elements
        if !self.show_body {
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([ratatui::layout::Constraint::Length(9), ratatui::layout::Constraint::Min(0)].as_ref())
                .split(area);

            let items: Vec<ListItem> = self.headers.iter().enumerate().map(|(i, header)| {
                let text = format!("{}: {} (prev: {})", header.key, header.value, header.previous_value);
                let style = if i == self.selected_header {
                    Style::default().fg(Color::White).bg(if is_active { Color::Blue } else { Color::default() }).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(text).style(style)
            }).collect();

            let title_spans = vec![
                Span::styled("Request ", Style::default().fg(if !self.show_body { Color::LightGreen } else { Color::White })),
                Span::styled("- Body ", Color::White),
            ];
            let title_text = Line::from(title_spans);

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(Line::from(title_text)))
                .style(Style::default().fg(if is_active { Color::Green } else { Color::White }))
                .highlight_style(Style::default().fg(if is_active { Color::Green } else { Color::White }).add_modifier(Modifier::BOLD)).highlight_symbol(" ");

            f.render_stateful_widget(list, chunks[0], &mut self.list_state.borrow_mut());
        }

        if self.show_body {
            let body_tab_spans: Vec<Span> = self.body_tabs.iter().enumerate().map(|(i, tab)| {
                let tab_text = tab.to_string();
                if i == self.selected_body_tab {
                    vec![
                        Span::styled(tab_text, Style::default().fg(Color::White).bg(if is_active { Color::Blue } else { Color::default() }).add_modifier(Modifier::BOLD)),
                        Span::raw(" ")
                    ]
                } else {
                    vec![
                        Span::styled(tab_text, Style::default().fg(Color::White)),
                        Span::raw(" ")
                    ]
                }
            }).flatten().collect();

            let body_tab_line = Line::from(body_tab_spans);

            let title_spans = vec![
                Span::styled("Request - ", Color::White),
                Span::styled("Body ", Style::default().fg(if self.show_body { Color::LightGreen } else { Color::White })),
            ];
            let title_line = Line::from(title_spans);

            let body_tabs_paragraph = Paragraph::new(Text::from(vec![body_tab_line]))
                .block(Block::default().borders(Borders::ALL).title(title_line).style(if is_active { Color::Green } else { Color::White }));

            f.render_widget(body_tabs_paragraph, chunks[0]);

            if !self.is_body_modal_open {

            
            
            let input_block = Block::default()
                .borders(Borders::ALL)
                .title("Body Input");

            let paragraph = Paragraph::new(self.inputs[0].value())
                .block(input_block)
                .style(if is_active && self.writable {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });

            f.render_widget(paragraph, chunks[1]);

            }

            if is_active && self.writable {
                f.set_cursor(
                    chunks[1].x + self.inputs[0].visual_cursor() as u16 + 1,
                    chunks[1].y + 1,
                );
            }
        }

        // Conditionally render modals over the main UI
        if self.is_modal_open {
            self.draw_modal::<B>(f);
        } else if self.is_body_modal_open {
            self.draw_body_modal::<B>(f);
        }

        if self.delete {
            let size = f.size();
            let menu_width = 30;
            let menu_height = 10;
            let menu_x = (size.width - menu_width) / 2;
            let menu_y = (size.height - menu_height) / 2;
            let area = Rect::new(menu_x, menu_y, menu_width, menu_height);

            let selection_block = Block::default()
                .borders(Borders::ALL)
                .title("Confirm Deletion")
                .style(Style::default().fg(Color::Red));

            let fields = vec!["Press Enter to Delete"];
            let field_list: Vec<ListItem> = fields.iter().map(|field| ListItem::new(*field)).collect();

            let list = List::new(field_list)
                .block(selection_block)
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(list, area, &mut self.list_state.borrow_mut());
        }
    }

    fn keybinds(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if self.delete {
                    self.delete_header();
                    self.delete = false;
                } else if self.is_modal_open {
                    if self.selected_input < 2 {
                        // Move to the next input field
                        self.selected_input += 1;
                    } else {
                        // Save the header if all input fields are filled
                        self.save_header();
                    }
                } else if self.show_body {
                    if self.body_tabs[self.selected_body_tab] != RequestHeaders::None {
                        if self.writable {
                            self.save_body();
                            self.is_body_modal_open = false;
                            self.writable = false;
                         
                           self.selected_body_tab += 1;
                        } else {
                            self.writable = true;
                            self.is_body_modal_open = true;
                        }
                    }
                } else if !self.writable && !self.adding_header && !self.is_editing {
                    // Open modal for editing header
                    self.is_modal_open = true;
                    self.is_editing = true;
                    let header = &self.headers[self.selected_header];
                    self.inputs[0] = Input::from(header.key.clone());
                    self.inputs[1] = Input::from(header.value.clone());
                    self.inputs[2] = Input::from(header.previous_value.clone());
                }
            }
            KeyCode::Esc => {
                if self.is_modal_open {
                    self.is_modal_open = false;
                    self.is_editing = false;
                    self.adding_header = false;
                    self.selected_input = 0;
                    self.inputs = [Input::default(), Input::default(), Input::default()];
                } else if self.is_body_modal_open{
                        self.is_body_modal_open = false;
                        self.writable = false;
                        self.inputs[0] = Input::default();
                        
                } else {
                    self.show_selection = false;
                    self.writable = false;
                    self.delete = false;
                    

                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                if !self.is_modal_open && !self.writable && !self.adding_header && !self.is_editing && !self.show_body {
                    self.adding_header = true;
                    self.is_modal_open = true;
                    self.selected_input = 0;
                    self.inputs = [Input::default(), Input::default(), Input::default()];
                } else {
                    if (self.writable) {
                        self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                    }
                }
            }
            KeyCode::Up => {
                /*   if self.is_modal_open && self.selected_input > 0 {
                       self.selected_input -= 1;
                   } */

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
                } else {
                    if self.selected_header > 0 {
                        self.selected_header -= 1;
                    } else {
                        self.selected_header = self.headers.len() - 1;
                    }
                    self.list_state.borrow_mut().select(Some(self.selected_header));
                }
            }
            KeyCode::Down => {
                /*    if self.is_modal_open && self.selected_input < 2 {
                       self.selected_input += 1;
                   } */

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
                } else {
                    if self.selected_header < self.headers.len() - 1 {
                        self.selected_header += 1;
                    } else {
                        self.selected_header = 0;
                    }
                    self.list_state.borrow_mut().select(Some(self.selected_header));
                }
            }
            KeyCode::Char('[') => {
                if !self.writable {
                    self.show_body = false;
                }
                self.inputs[self.selected_input] = Input::default();
            }
            KeyCode::Char(']') => {
                if !self.writable {
                    self.show_body = true;
                    self.load_body();
                }
                self.inputs[self.selected_input] = Input::default();
            }
            KeyCode::Char('q') => {
                if self.writable || self.adding_header || self.is_editing {
                    self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                }
                return;
            }
            KeyCode::Left => {
                if self.delete   {

                  }  else if self.show_selection {

                } else if self.writable || self.adding_header || self.is_editing || self.is_body_modal_open  {
                    self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                } else if self.show_body {
                    if self.selected_body_tab > 0 {
                        self.selected_body_tab -= 1;
                    } else {
                        self.selected_body_tab = self.body_tabs.len() - 1;
                    }
                    self.load_body();
                }
                
                
                 else if !self.writable {
                    if self.selected_header > 0 {
                        self.selected_header -= 1;
  
                    } else {
                        self.selected_header = self.headers.len() - 1;
                    }
                }
            }
            KeyCode::Right => {
                if self.writable || self.adding_header || self.is_editing || self.is_body_modal_open  {
                    self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                } 
               else  if self.show_body || self.is_modal_open  {
                    if self.selected_body_tab < self.body_tabs.len() - 1 {
                        self.selected_body_tab += 1;
                    } else {
                        self.selected_body_tab = 0;
                    }
                    self.load_body();
                } else if self.show_selection {

                }  else if !self.writable {
                    if self.selected_header < self.headers.len() - 1 {
                        self.selected_header += 1;
                    } else {
                        self.selected_header = 0;
                    }
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {


                if self.show_body{

                        if self.writable {
                            self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                        }
                } else{               
                    if !self.writable && !self.adding_header && !self.is_editing   {
                        self.delete = true;
                    } else {
                        self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                    }
                }
                
            }
            KeyCode::Tab => {
                if self.is_modal_open {
                    if self.selected_input < 2 {
                        // Move to the next input field
                        self.selected_input += 1;
                    } else {
                        self.selected_input = 0;
                    }
                }
            }
            KeyCode::BackTab => {
                if self.is_modal_open {
                    if self.selected_input > 0 {
                        self.selected_input -= 1;
                    } else {
                        self.selected_input = 2;
                    }
                }
            }
            _ => {
                if self.writable || self.adding_header || self.is_editing {
                    self.inputs[self.selected_input].handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
                }
            }
        }
    }
}
