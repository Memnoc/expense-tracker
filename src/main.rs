mod db;
mod expense;

use chrono::NaiveDate;
use db::Database;
use expense::Expense;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::new().await?;

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ")?;

        match choice.as_str() {
            "1" => add_expense(&database).await?,
            "2" => list_expenses(&database).await?,
            "3" => update_expense(&database).await?,
            "4" => delete_expense(&database).await?,
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}

fn print_menu() {
    println!("\nExpense Tracker Menu:");
    println!("1. Add Expense");
    println!("2. List Expenses");
    println!("3. Update Expense");
    println!("4. Delete Expense");
    println!("5. Exit");
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

async fn add_expense(database: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let date = get_user_input("Enter date (YYYY-MM-DD): ")?;
    let name = get_user_input("Enter expense name: ")?;
    let category = get_user_input("Enter category: ")?;
    let amount = get_user_input("Enter amount: ")?.parse::<f64>()?;

    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
    let expense = Expense::new(date, &name, &category, amount)?;

    database.insert_expense(&expense).await?;
    println!("Expense added successfully!");
    Ok(())
}

async fn list_expenses(database: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let expenses = database.list_expenses().await?;
    for expense in expenses {
        println!(
            "ID: {:?}, Date: {}, Name: {}, Category: {}, Amount: {}",
            expense.id, expense.date, expense.name, expense.category, expense.amount
        );
    }
    Ok(())
}

async fn update_expense(database: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let id = get_user_input("Enter expense ID to update: ")?.parse::<i64>()?;
    let mut expense = match database.get_expense(id).await? {
        Some(e) => e,
        None => {
            println!("Expense not found!");
            return Ok(());
        }
    };

    let date = get_user_input("Enter new date (YYYY-MM-DD): ")?;
    let name = get_user_input("Enter new name: ")?;
    let category = get_user_input("Enter new category: ")?;
    let amount = get_user_input("Enter new amount: ")?.parse::<f64>()?;

    expense.date = date;
    expense.name = name;
    expense.category = category;
    expense.amount = amount;

    database.update_expense(&expense).await?;
    println!("Expense updated successfully!");
    Ok(())
}

async fn delete_expense(database: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let id = get_user_input("Enter expense ID to delete: ")?.parse::<i64>()?;
    database.delete_expense(id).await?;
    println!("Expense deleted successfully!");
    Ok(())
}
