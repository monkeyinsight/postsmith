use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::size;

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::components::InputComponent;
use crate::ui::Component;

pub struct InputModalComponent {
    pub input: Input,
    pub show_modal: bool,
}

impl InputModalComponent {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
            show_modal: false,
        }
    }
    pub fn capture_input(&mut self) -> String {
        self.input.value().to_string()
    }
    pub fn pass_url(&mut self, input_component: &mut InputComponent) {
        let url = self.capture_input();
        input_component.value = url;
    }
    pub fn draw_modal<B: Backend>(&self, f: &mut Frame, is_active: bool) {
        let terminal_size = size().unwrap();
        let modal_width = terminal_size.0 - 10;
        let modal_height = terminal_size.1 - 10;
        let modal_area = Rect::new(
            (terminal_size.0 - modal_width) / 2,
            (terminal_size.1 - modal_height) / 2,
            modal_width,
            modal_height,
        );

        let modal_block = Block::default()
            .title("URL Input")
            .borders(Borders::ALL)
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }).bg(Color::Black));

        f.render_widget(modal_block, modal_area);

        let input_area = Rect::new(
            modal_area.x + 2,
            modal_area.y + 2,
            modal_area.width - 4,
            modal_area.height - 4,
        );

        let paragraph = Paragraph::new(self.input.value())
            .block(Block::default().borders(Borders::ALL).title("URL"))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(paragraph, input_area);

        let cursor_position = self.input.visual_cursor();
        let wrapped_text = self.input.value().chars().collect::<Vec<_>>();
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

            f.set_cursor(cursor_x, cursor_y);
        }
    }
}

impl Component for InputModalComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, _area: Rect, is_active: bool) {
        if self.show_modal {
            self.draw_modal::<B>(f, is_active);
        }
    }

    fn keybinds(&mut self, key: KeyCode) {
        self.input.handle_event(&Event::Key(KeyEvent::new(
            key,
            crossterm::event::KeyModifiers::NONE,
        )));
    }
}
