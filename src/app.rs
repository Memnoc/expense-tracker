use crate::expense::Expense;

#[derive(Clone, Copy, PartialEq)]
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

impl Default for App {
    fn default() -> Self {
        Self {
            expenses: Vec::new(),
            selected_index: None,
            adding_expense: false,
            new_expense: Expense::new(chrono::Local::now().date_naive(), "", "", 0.0).unwrap(),
            input_mode: InputMode::Date,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_expenses(&mut self, expenses: Vec<Expense>) {
        self.expenses = expenses;
    }
}
