mod db;
mod expense;

use chrono::NaiveDate;
use expense::Expense;

#[tokio::main]
async fn main() {
    let expense = Expense::new(
        NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
        "Grocery shopping",
        "food",
        50.0,
    )
    .unwrap();
    println!("Created expense: {:?}", expense);

    // We'll add TUI code here later
}
