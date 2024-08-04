mod db;
mod expense;
mod ui;

use crate::db::Database;
use crate::expense::Expense;
use chrono::NaiveDate;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, widgets::Dataset, Terminal};
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

    // Create a DB connection
    let db = Database::new().await?;

    // Add a test expense
    let test_expense = Expense::new(
        NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        "Test Expense",
        "Food",
        50.0,
    )
    .unwrap();
    db.insert_expense(&test_expense).await?;
    println!("Added test expense");

    // Create app and run it
    let app = App::new();
    let res = run_app::<CrosstermBackend<io::Stdout>>(&mut terminal, app, db).await;

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
    db: Database,
) -> io::Result<()> {
    // Fetch expenses once at the start
    let expenses = db.list_expenses().await.unwrap();
    println!("Fetched {} expenses", expenses.len());
    app.set_expenses(expenses);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => {
                    // TODO: Implement add expense functionality
                    println!("Add expense functionality not implemented yet");
                }
                KeyCode::Char('d') => {
                    // TODO: Implement delete expense functionality
                    println!("Delete expense functionality not implemented yet");
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

            // Refresh expenses list only after an action that might change it
            if matches!(key.code, KeyCode::Char('a') | KeyCode::Char('d')) {
                let expenses = db.list_expenses().await.unwrap();
                println!("Fetched {} expenses after action", expenses.len());
                app.set_expenses(expenses);
            }
        }
    }
}
