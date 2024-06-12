use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;

use crossterm::event::{KeyCode, KeyEvent, Event};

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::ui::Component;
use crate::components::InputComponent;


pub struct InputModalComponent {
    pub input: Input,
}

impl InputModalComponent {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
        }
    }
    pub fn capture_input(&mut self) -> String {
        self.input.value().to_string()
    }
    pub fn pass_url(&mut self, input_component: &mut InputComponent) {
        let url = self.capture_input();
        input_component.value = url;
        
    }
}

impl Component for InputModalComponent {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("URL Input")
            .style(Style::default().fg(if is_active {
                Color::Green
            } else {
                Color::White
            }));

        let paragraph = Paragraph::new(self.input.value())
            .block(block)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(paragraph, area);

        if is_active {
            f.set_cursor(
                area.x + self.input.visual_cursor() as u16 + 1,
                area.y + 1,
            )
        }
    }

    fn keybinds(&mut self, key: KeyCode) {
        self.input.handle_event(&Event::Key(KeyEvent::new(key, crossterm::event::KeyModifiers::NONE)));
    }
}