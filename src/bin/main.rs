 use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    Terminal,
    Frame,
    text::{Span, Spans}
};

use tokio::runtime::Runtime;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum HttpMethod{
    GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
}

impl HttpMethod {
    fn all_methods() -> Vec<HttpMethod> {
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

    fn to_string(&self) -> &'static str {
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



#[derive(Debug, PartialEq, Eq)]
enum ActiveBlock {
    Method,
    Input,
    Message,
    MethodSelection,

}

struct AppState {
    method: HttpMethod,
    input: String,
    message: String,
    input_x: u16,
    input_y: u16, 
    message_x: u16,
    message_y: u16,
    active_block: ActiveBlock,
    show_method_selection: bool,
    list_state: ListState,
}

impl AppState {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            method: HttpMethod::GET,
            input: String::new(),
            message: String::new(),
             input_x: 0,
            input_y: 0, 
            message_x: 0,
            message_y: 0,
            active_block: ActiveBlock::Input,
            show_method_selection: false,
            list_state,
           
        }
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application state
    let mut app_state = AppState::new();

    // Run the application loop
    let res = run_app(&mut terminal, &mut app_state);

    

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) -> io::Result<()> {
    let rt = Runtime::new().unwrap();
   

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(80),
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

            // Method block
            let method_block = Block::default()
                .borders(Borders::ALL)
                .title("Method")
                .style(Style::default().fg(if let ActiveBlock::Method = app_state.active_block {
                    Color::Green
                } else {
                    Color::White
                }));
            let method_paragraph = Paragraph::new(format!("{:?}", app_state.method))
                .block(method_block)
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            f.render_widget(method_paragraph, top_chunks[0]);

            // Input block
            let input_block = Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .style(Style::default().fg(if let ActiveBlock::Input = app_state.active_block {
                    Color::Green
                } else {
                    Color::White
                }));
                let input_text = app_state.input
                .lines()
                .map(|line| Spans::from(Span::raw(line)))
                .collect::<Vec<Spans>>();

                let paragraph = Paragraph::new(input_text)
                .block(input_block)
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .wrap(tui::widgets::Wrap { trim: false }).scroll((app_state.input_y, app_state.input_x));;
            f.render_widget(paragraph, top_chunks[1]);

            // Message block
            let message_block = Block::default()
                .borders(Borders::ALL)
                .title("Message")
                .style(Style::default().fg(if let ActiveBlock::Message = app_state.active_block {
                    Color::Green
                } else {
                    Color::White
                }));
            let message_paragraph = Paragraph::new(app_state.message.clone())
                .block(message_block)
                .style(Style::default().fg(Color::Green)).scroll((app_state.message_y, app_state.message_x));
            f.render_widget(message_paragraph, chunks[1]);
        
            if app_state.show_method_selection {
                render_method_selection_menu(f, app_state, size);
            }
        
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if let ActiveBlock::Input = app_state.active_block {
                        app_state.input.push(c);
                    }
                   
                }
                KeyCode::Backspace => {
                    if let ActiveBlock::Input = app_state.active_block {
                        app_state.input.pop();
                    }

                }
                KeyCode::Enter => {

                    if app_state.active_block == ActiveBlock::Method {
                        app_state.show_method_selection = true;
                        app_state.active_block = ActiveBlock::MethodSelection;
                    }

                   else if app_state.show_method_selection {
                        if let Some(selected) = app_state.list_state.selected() {
                            app_state.method = HttpMethod::all_methods()[selected];
                        }
                        app_state.show_method_selection = false;
                        app_state.active_block = ActiveBlock::Method;
                    } else if let ActiveBlock::Input = app_state.active_block {
                        let url = app_state.input.clone();
                       app_state.input_x = 0;
                        app_state.input_y = 0; 
                        if app_state.method ==  HttpMethod::GET{

                        }
                        let response = rt.block_on(send_get_request(&url));
                        match response {
                            Ok(body) => app_state.message = body,
                            Err(err) => app_state.message = format!("Error: {}", err),
                        }
                    }
                }
                KeyCode::Esc => {
                        if app_state.active_block == ActiveBlock::MethodSelection {
                                app_state.active_block = ActiveBlock::Method;
                                app_state.show_method_selection = false;
                        } else{
                            return Ok(());
                        }

                  
                }
                KeyCode::Up => {
                    if app_state.show_method_selection {
                        let i = match app_state.list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    HttpMethod::all_methods().len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        app_state.list_state.select(Some(i));
                        }    else {
                    match app_state.active_block {
                        ActiveBlock::Input => {
                            if app_state.input_y > 0 {
                                app_state.input_y -= 1;
                            } 
                        }
                        ActiveBlock::Message => {
                            if app_state.message_y > 0 {
                                app_state.message_y -= 1;
                            }
                        }
                    
                        _ => {}
                    }
                }
                    
                }
                KeyCode::Down => {
                    if app_state.show_method_selection {
                        let i = match app_state.list_state.selected() {
                            Some(i) => {
                                if i == HttpMethod::all_methods().len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        app_state.list_state.select(Some(i));
                    } else {
                        match app_state.active_block {
                           ActiveBlock::Input => app_state.input_y += 1,
                            ActiveBlock::Message => app_state.message_y += 1,
                            _ => {}
                        }
                    }

                }

                   
                    


                
                KeyCode::Left => {
                    match app_state.active_block {
                        ActiveBlock::Input => {
                             if app_state.input_x > 0 {
                                app_state.input_x -= 1;
                            } 
                        }
                        ActiveBlock::Message => {
                            if app_state.message_x > 0 {
                                app_state.message_x -= 1;
                            }
                        }
                        _ => {}
                    }
                    
                }
                KeyCode::Right => {
                    match app_state.active_block {
                         ActiveBlock::Input => app_state.input_x += 1, 
                        ActiveBlock::Message => app_state.message_x += 1,
                        _ => {}
                    }
                  
                }
                KeyCode::Tab => {

                    if app_state.show_method_selection == false {
                        app_state.active_block = match app_state.active_block {
                            ActiveBlock::Method => ActiveBlock::Input,
                                ActiveBlock::Input => ActiveBlock::Message,
                                ActiveBlock::Message => ActiveBlock::Method,
                                ActiveBlock::MethodSelection => ActiveBlock::Method,
                            
                           
                        };
                    }
                    
                   
                }
                

                
              
                
                _ => {}
            }
        }
    }
} 

fn render_method_selection_menu<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState, size: Rect) {
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

    f.render_stateful_widget(methods, area, &mut app_state.list_state);
}

async fn send_get_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)    
}