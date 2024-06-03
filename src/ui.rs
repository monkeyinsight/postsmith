use crossterm::event::KeyCode;
use ratatui::{backend::Backend, layout::{Constraint, Direction, Layout, Rect}, Terminal, Frame};
use crate::components::{InputComponent, OutputComponent, SelectorComponent};
use crate::session::Session;

pub trait Component {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool);
    fn keybinds(&mut self, key: KeyCode);
}

pub struct AppState {
    pub method_component: SelectorComponent,
    pub input_component: InputComponent,
    pub message_component: OutputComponent,
    pub active_block: ActiveBlock,
    pub runtime: tokio::runtime::Runtime,
    pub session: Session,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ActiveBlock {
    Method,
    Input,
    Message,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            method_component: SelectorComponent::new(),
            input_component: InputComponent::new(),
            message_component: OutputComponent::new(),
            active_block: ActiveBlock::Method,
            runtime: tokio::runtime::Runtime::new().unwrap(),
            session: Session::new(),
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode) -> bool {
        match self.active_block {
            ActiveBlock::Method => self.method_component.keybinds(key),
            ActiveBlock::Input => self.input_component.keybinds(key),
            ActiveBlock::Message => self.message_component.keybinds(key),
        }

        if key == KeyCode::BackTab {
            self.active_block = match self.active_block {
                ActiveBlock::Method => ActiveBlock::Message,
                ActiveBlock::Input => ActiveBlock::Method,
                ActiveBlock::Message => ActiveBlock::Input,
            }
        } else if key == KeyCode::Tab {
            self.active_block = match self.active_block {
                ActiveBlock::Method => ActiveBlock::Input,
                ActiveBlock::Input => ActiveBlock::Message,
                ActiveBlock::Message => ActiveBlock::Method,
            };
        } else if key == KeyCode::Enter {
            if self.active_block == ActiveBlock::Input {
                let url = self.input_component.input.clone();
                let response = self.runtime.block_on(crate::request::send_get_request(&url.value()));

                self.session.push_history(url.to_string());
                match response {
                    Ok(body) => self.message_component.message = body,
                    Err(err) => self.message_component.message = format!("Error: {}", err),
                }
            }
        } else if key == KeyCode::Char('q') {
            if self.active_block != ActiveBlock::Input {
                return true;
            }
        }

        false
    }
}

pub fn draw_ui<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) -> std::io::Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(size);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        app_state.method_component.draw::<B>(f, top_chunks[0], app_state.active_block == ActiveBlock::Method);
        app_state.input_component.draw::<B>(f, top_chunks[1], app_state.active_block == ActiveBlock::Input);
        app_state.message_component.draw::<B>(f, chunks[1], app_state.active_block == ActiveBlock::Message);
    })?;
    Ok(())
}
