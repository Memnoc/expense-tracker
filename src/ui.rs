use crate::expense::Expense;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub enum InputMode {
    Date,
    Name,
    Category,
    Amount,
}
pub struct App {
    pub expenses: Vec<Expense>,
    pub selected_index: Option<usize>,
    pub adding_expense: bool,
    pub new_expense: Expense,
    pub input_mode: InputMode,
}

impl App {
    pub fn new() -> App {
        App {
            expenses: Vec::new(),
            selected_index: None,
            adding_expense: false,
            new_expense: Expense::new(chrono::Local::now().date_naive(), "", "", 0.0).unwrap(),
            input_mode: InputMode::Date,
        }
    }

    pub fn set_expenses(&mut self, expenses: Vec<Expense>) {
        self.expenses = expenses;
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("Expense Tracker")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let expenses: Vec<ListItem> = app
        .expenses
        .iter()
        .map(|expense| {
            let content = Line::from(vec![
                Span::styled(
                    format!("{:<10}", expense.date),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{:<20}", expense.name)),
                Span::raw(format!("{:<15}", expense.category)),
                Span::styled(
                    format!("${:.2}", expense.amount),
                    Style::default().fg(Color::Green),
                ),
            ]);
            ListItem::new(Text::from(content))
        })
        .collect();

    if app.adding_expense {
        let input = Paragraph::new(format!(
            "Date: {}\nName: {}\nCategory: {}\nAmount: {}",
            app.new_expense.date,
            app.new_expense.name,
            app.new_expense.category,
            app.new_expense.amount
        ))
        .block(Block::default().borders(Borders::ALL).title("New Expense"));
        f.render_widget(input, chunks[1]);
    } else {
        let expenses_list = List::new(expenses)
            .block(Block::default().borders(Borders::ALL).title("Expenses"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_widget(expenses_list, chunks[1]);
    }

    let footer = Paragraph::new("Press 'q' to quit, 'a' to add expense, 'd' to delete expense")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
