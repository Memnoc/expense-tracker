mod app;
mod db;
mod expense;
mod ui;

use crate::app::{App, InputMode};
use crate::db::Database;
use crate::ui::ui;
use chrono::NaiveDate;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use expense::Expense;
use ratatui::backend::Backend;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //INFO: Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //INFO: Create a DB connection
    let db = Database::new().await?;

    //INFO: Add a test expense
    let test_expense = Expense::new(
        NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        "Test Expense",
        "Food",
        50.0,
    )
    .unwrap();
    db.insert_expense(&test_expense).await?;
    println!("Added test expense");

    //INFO: Create app and run it
    let app = App::new();
    let res = run_app::<CrosstermBackend<io::Stdout>>(&mut terminal, app, db).await;

    //INFO: Restore terminal
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

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    db: Database,
) -> io::Result<()> {
    // // Fetch expenses once at the start
    // let expenses = db.list_expenses().await.unwrap();
    // println!("Fetched {} expenses", expenses.len());
    // app.set_expenses(expenses);

    loop {
        terminal.draw(|f| ui::<CrosstermBackend<io::Stdout>>(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => {
                    app.adding_expense = true;
                    app.new_expense =
                        Expense::new(chrono::Local::now().date_naive(), "", "", 0.0).unwrap();
                }
                KeyCode::Char('d') => {
                    if let Some(selected) = app.selected_index {
                        if let Some(expense) = app.expenses.get(selected) {
                            if let Some(id) = expense.id {
                                db.delete_expense(id).await.unwrap();
                                // print!("Deleted expense with id: {}", id);
                                // Refresh the list after the deletion
                                // let expenses = db.list_expenses().await.unwrap();
                                // app.set_expenses(expenses);
                                app.expenses = db.list_expenses().await.unwrap();
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    if app.adding_expense {
                        db.insert_expense(&app.new_expense).await.unwrap();
                        println!("Added new expense: {}", app.new_expense.name);
                        app.adding_expense = false;
                        // Refresh the list after insertion
                        let expenses = db.list_expenses().await.unwrap();
                        app.set_expenses(expenses);
                    }
                }
                KeyCode::Char(c) => {
                    if app.adding_expense {
                        match app.input_mode {
                            InputMode::Date => app.new_expense.date.push(c),
                            InputMode::Name => app.new_expense.name.push(c),
                            InputMode::Category => app.new_expense.category.push(c),
                            InputMode::Amount => {
                                if c.is_ascii_digit() || c == '.' {
                                    let mut amount_str = app.new_expense.amount.to_string();
                                    amount_str.push(c);
                                    if let Ok(amount) = amount_str.parse() {
                                        app.new_expense.amount = amount;
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if app.adding_expense {
                        match app.input_mode {
                            InputMode::Date => {
                                app.new_expense.date.pop();
                            }
                            InputMode::Name => {
                                app.new_expense.name.pop();
                            }
                            InputMode::Category => {
                                app.new_expense.category.pop();
                            }
                            InputMode::Amount => {
                                let mut amount_str = app.new_expense.amount.to_string();
                                amount_str.pop();
                                app.new_expense.amount = amount_str.parse().unwrap_or(0.0);
                            }
                        }
                    }
                }
                KeyCode::Tab => {
                    //INFO: cycle through input modes
                    app.input_mode = match app.input_mode {
                        InputMode::Date => InputMode::Name,
                        InputMode::Name => InputMode::Category,
                        InputMode::Category => InputMode::Amount,
                        InputMode::Amount => InputMode::Date,
                    };
                }
                KeyCode::Esc => {
                    app.adding_expense = false;
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
                // println!("Fetched {} expenses after action", expenses.len());
                app.set_expenses(expenses);
            }
        }
    }
}
