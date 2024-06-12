use crossterm::event::KeyCode;
use ratatui::{backend::Backend, layout::{Constraint, Direction, Layout, Rect}, Terminal, Frame};
use crate::components::{InputComponent, OutputComponent, SelectorComponent, RequestComponent};

pub trait Component {
    fn draw<B: Backend>(&self, f: &mut Frame, area: Rect, is_active: bool);
    fn keybinds(&mut self, key: KeyCode);
}

pub struct AppState {
    pub method_component: SelectorComponent,
    pub input_component: InputComponent,
    pub message_component: OutputComponent,
    pub active_block: ActiveBlock,
    pub request_component: RequestComponent,
    pub runtime: tokio::runtime::Runtime,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ActiveBlock {
    Method,
    Input,
    Message,
    Request
}

impl AppState {
    pub fn new() -> Self {
        Self {
            method_component: SelectorComponent::new(),
            input_component: InputComponent::new(),
            message_component: OutputComponent::new(),
            active_block: ActiveBlock::Method,
            request_component: RequestComponent::new(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode) -> bool {
        match self.active_block {
            ActiveBlock::Method => self.method_component.keybinds(key),
            ActiveBlock::Input => self.input_component.keybinds(key),
            ActiveBlock::Message => self.message_component.keybinds(key),
            ActiveBlock::Request  => self.request_component.keybinds(key)
        }

        if key == KeyCode::BackTab {
            if !self.request_component.is_modal_open {
                self.active_block = match self.active_block {
                    ActiveBlock::Method => ActiveBlock::Message,
                    ActiveBlock::Input => ActiveBlock::Method,
                    ActiveBlock::Message => ActiveBlock::Request,
                    ActiveBlock::Request => ActiveBlock::Input,
                }
            }
           
        } else if key == KeyCode::Tab {
            if !self.request_component.is_modal_open {
            self.active_block = match self.active_block {
                ActiveBlock::Method => ActiveBlock::Input,
                ActiveBlock::Input => ActiveBlock::Request,
                ActiveBlock::Request => ActiveBlock::Message,
                ActiveBlock::Message => ActiveBlock::Method,
            };
        }
        } else if key == KeyCode::Enter {
            if self.active_block == ActiveBlock::Input {
                let url = self.input_component.input.clone();
                let response = self.runtime.block_on(crate::request::send_get_request(&url.value()));
                match response {
                    Ok(body) => self.message_component.message = body,
                    Err(err) => self.message_component.message = format!("Error: {}", err),
                }
            }
        } else if key == KeyCode::Char('q') {
            if self.active_block != ActiveBlock::Input && !self.request_component.adding_header && !self.request_component.writable {
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
                    Constraint::Length(6),
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
        app_state.request_component.draw::<B>(f, chunks[1], app_state.active_block == ActiveBlock::Request);
        app_state.message_component.draw::<B>(f, chunks[2], app_state.active_block == ActiveBlock::Message);
    })?;
    Ok(())
}
