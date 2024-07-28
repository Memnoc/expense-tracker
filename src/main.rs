mod expense;

use chrono::NaiveDate;
use expense::Expense;

fn main() {
    println!("==Expense Tracker==");
    let expense = Expense::new(
        NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
        "Grocery Shopping",
        "food",
        50.0,
    );
    println!("Created expense: {:?}", expense);
}
