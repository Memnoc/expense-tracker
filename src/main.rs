mod db;
mod expense;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};
use ui::{ui, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new();
    let res = run_app::<CrosstermBackend<io::Stdout>>(&mut terminal, app).await;

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

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    let db = db::Database::new().await.unwrap();

    loop {
        terminal.draw(|f| ui::<B>(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => {
                    // TODO: Implement add expense functionality
                }
                KeyCode::Char('d') => {
                    // TODO: Implement delete expense functionality
                }
                KeyCode::Up => {
                    if let Some(selected) = app.selected_index {
                        if selected > 0 {
                            app.selected_index = Some(selected - 1);
                        }
                    } else {
                        app.selected_index = Some(0);
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = app.selected_index {
                        if selected < app.expenses.len().saturating_sub(1) {
                            app.selected_index = Some(selected + 1);
                        }
                    } else {
                        app.selected_index = Some(0);
                    }
                }
                _ => {}
            }
        }

        // Refresh expenses list
        app.set_expenses(db.list_expenses().await.unwrap());
    }
}
