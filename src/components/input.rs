use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::Frame;

use crossterm::event::{KeyCode, KeyEvent,  Event};

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::ui::Component;

pub struct InputComponent {
    pub input: Input,
    
}

impl InputComponent {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
            
        }
    }
}

impl Component for InputComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_active: bool) {
        let block = Block::default()
        .borders(Borders::ALL)
        .title("Input")
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
