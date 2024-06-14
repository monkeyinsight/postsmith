use crossterm::terminal::size;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;

use crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::ui::Component;

pub struct InputComponent {
    pub value: String,
    pub input: Input,
    pub show_modal: bool,
}

impl InputComponent {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            input: Input::default(),
            show_modal: false,

        }
    }

    pub fn capture_input(&mut self) -> String {
        self.input.value().to_string()
    }
    pub fn pass_url(&mut self) {
        let url = self.capture_input();
        self.value = url;
    }
    pub fn draw_modal<B: Backend>(&self, f: &mut Frame, is_active: bool) {
        let terminal_size = size().unwrap();
        let modal_width = 80;
        let modal_height = 5;
        let modal_area = Rect::new(
            (terminal_size.0 - modal_width) / 2,
            (terminal_size.1 - modal_height) / 2,
            modal_width,
            modal_height,
        );

        let input_area = Rect::new(
            modal_area.x + 1,
            modal_area.y + 1,
            modal_area.width - 2,
            modal_area.height - 2,
        );

        let paragraph = Paragraph::new(self.input.value())
        .block(Block::default().borders(Borders::ALL).title("URL Input"))
        .wrap(ratatui::widgets::Wrap { trim: false })
        .style(
            Style::default()
                .fg(if is_active { Color::Green } else { Color::White })
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(paragraph, modal_area);

    let cursor_position = self.input.visual_cursor();
    let wrapped_text = self.input.value().chars().collect::<Vec<_>>();
    let mut cursor_x = input_area.x; 
    let mut cursor_y = input_area.y;
    let mut line_length = 0;

    for (index, ch) in wrapped_text.iter().enumerate() {
        if index == cursor_position {
            break;
        }
        if *ch == '\n' || line_length >= modal_area.width as usize - 2 {
            cursor_y += 1;
            cursor_x = input_area.x;
            line_length = 0;
        } else {
            cursor_x += 1;
            line_length += 1;
        }
    }

    f.set_cursor(cursor_x, cursor_y);
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

        if self.show_modal {
            self.draw_modal::<B>(f, is_active);
        }
    }

    fn keybinds(&mut self, _key: KeyCode) {

        match _key {
            KeyCode::Char('e') => {
                if self.show_modal {
                    self.input.handle_event(&Event::Key(KeyEvent::new(
                        _key,
                        crossterm::event::KeyModifiers::NONE,
                    )));
                } else{
                    self.show_modal = true;
                }
            }

            KeyCode::Enter => {

                if self.show_modal{
                    self.pass_url();
                    self.show_modal = false;
                } else {
                    self.show_modal = true;
                }
               
            }

            KeyCode::Esc => {
                self.show_modal = false;
            }
            _ => {
                if self.show_modal {
                    self.input.handle_event(&Event::Key(KeyEvent::new(
                        _key,
                        crossterm::event::KeyModifiers::NONE,
                    )));
                }
            }

        }

    }
}
