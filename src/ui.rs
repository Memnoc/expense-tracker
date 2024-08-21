use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{App, InputMode};

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("Expense Tracker")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if app.adding_expense {
        render_add_expense_form(f, app, chunks[1]);
    } else {
        render_expense_list(f, app, chunks[1]);
    }

    let mut footer_text = "Press 'q' to quit, 'a' to add expense".to_string();
    if !app.expenses.is_empty() {
        footer_text.push_str(", 'up/down' to select, 'd' to delete expense");
    }
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_add_expense_form(f: &mut Frame, app: &App, area: Rect) {
    let input_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    let date_input = Paragraph::new(app.new_expense.date.as_str())
        .style(input_style(app.input_mode == InputMode::Date))
        .block(Block::default().borders(Borders::ALL).title("Date"));
    f.render_widget(date_input, input_chunks[0]);

    let name_input = Paragraph::new(app.new_expense.name.as_str())
        .style(input_style(app.input_mode == InputMode::Name))
        .block(Block::default().borders(Borders::ALL).title("Name"));
    f.render_widget(name_input, input_chunks[1]);

    let category_input = Paragraph::new(app.new_expense.category.as_str())
        .style(input_style(app.input_mode == InputMode::Category))
        .block(Block::default().borders(Borders::ALL).title("Category"));
    f.render_widget(category_input, input_chunks[2]);

    let amount_input = Paragraph::new(app.new_expense.amount.to_string())
        .style(input_style(app.input_mode == InputMode::Amount))
        .block(Block::default().borders(Borders::ALL).title("Amount"));
    f.render_widget(amount_input, input_chunks[3]);
}

fn render_expense_list(f: &mut Frame, app: &App, area: Rect) {
    let expenses: Vec<ListItem> = app
        .expenses
        .iter()
        .enumerate()
        .map(|(index, expense)| {
            let content = Line::from(vec![
                Span::styled(
                    format!("{:<12}", expense.date),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("{:<20}", expense.name), Style::default()),
                Span::styled(format!("{:<15}", expense.category), Style::default()),
                Span::styled(format!("{:<15}", expense.amount), Style::default()),
            ]);
            if Some(index) == app.selected_index {
                ListItem::new(content).style(Style::default().bg(Color::DarkGray))
            } else {
                ListItem::new(content)
            }
        })
        .collect();

    let expenses_list = List::new(expenses)
        .block(Block::default().borders(Borders::ALL).title("Expenses"))
        .highlight_style(Style::default().bg(Color::DarkGray));

    let mut state = ListState::default();
    state.select(app.selected_index);
    f.render_stateful_widget(expenses_list, area, &mut state);
}

fn input_style(is_selected: bool) -> Style {
    if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    }
}
