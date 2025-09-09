use anyhow::Result;
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod terminal;
mod ai;
mod ui;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new().await;
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Tab if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        app.next_tab();
                    }
                    KeyCode::Char('t') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        app.new_tab().await;
                    }
                    KeyCode::Char('w') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        app.close_current_tab();
                    }
                    KeyCode::Enter => {
                        app.execute_current_input().await;
                    }
                    KeyCode::Backspace => {
                        app.handle_backspace();
                    }
                    KeyCode::Tab => {
                        app.handle_tab_completion().await;
                    }
                    KeyCode::Char(c) => {
                        app.handle_char_input(c);
                    }
                    _ => {}
                }
            }
        }
    }
}