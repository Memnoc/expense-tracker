use std::fs;

use crate::expense::Expense;
use sqlx::{query, query_as, sqlite::SqlitePool, Pool, Sqlite};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS expenses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                amount REAL NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Database { pool })
    }

    pub async fn insert_expense(&self, expense: &Expense) -> Result<i64, sqlx::Error> {
        let result =
            sqlx::query("INSERT INTO expenses (date, name, category, amount) VALUES (?, ?, ?, ?)")
                .bind(expense.date.to_string())
                .bind(&expense.name)
                .bind(&expense.category)
                .bind(expense.amount)
                .execute(&self.pool)
                .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_expense(&self, id: i64) -> Result<Option<Expense>, sqlx::Error> {
        query_as::<_, Expense>("SELECT id, date, name, category, amount FROM expenses WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update_expense(&self, expense: &Expense) -> Result<(), sqlx::Error> {
        query("UPDATE expenses SET date = ?, name = ?, category = ?, amount = ? WHERE id = ?")
            .bind(expense.date.to_string())
            .bind(&expense.name)
            .bind(&expense.category)
            .bind(expense.amount)
            .bind(expense.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_expense(&self, id: i64) -> Result<(), sqlx::Error> {
        query("DELETE FROM expenses WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn clear_expenses(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM expenses")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_expenses(&self) -> Result<Vec<Expense>, sqlx::Error> {
        query_as::<_, Expense>("SELECT id, date, name, category, amount FROM expenses")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn save_expenses_to_file(
        &self,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expenses = self.list_expenses().await?;
        let json = serde_json::to_string(&expenses)?;
        fs::write(filename, json)?;
        Ok(())
    }

    pub async fn load_expenses_from_file(
        &self,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(contents) = fs::read_to_string(filename) {
            let expenses: Vec<Expense> = serde_json::from_str(&contents)?;
            for expense in expenses {
                self.insert_expense(&expense).await?;
            }
        }
        Ok(())
    }

    pub async fn filter_by_category(&self, category: &str) -> Result<Vec<Expense>, sqlx::Error> {
        sqlx::query_as::<_, Expense>(
            "SELECT id, date, name, category, amount FROM expenses WHERE category = ?",
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn filter_by_month(
        &self,
        year: i32,
        month: u32,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let start_date = format!("{:04}-{:02}-01", year, month);
        let end_date = format!("{:04}-{:02}-31", year, month);

        sqlx::query_as::<_, Expense>(
            "SELECT id, date, name, category, amount FROM expenses WHERE date >= ? AND date <= ?",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use tempfile::NamedTempFile;

    #[tokio::test]

    async fn test_save_and_laod_expenses() {
        let db = Database::new().await.unwrap();

        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Create some test expenses
        let expense1 = Expense::new(
            chrono::NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
            "Test 1",
            "Food",
            50.0,
        )
        .unwrap();
        let expense2 = Expense::new(
            chrono::NaiveDate::from_ymd_opt(2023, 7, 2).unwrap(),
            "Test 2",
            "Transport",
            30.0,
        )
        .unwrap();

        db.insert_expense(&expense1).await.unwrap();
        db.insert_expense(&expense2).await.unwrap();

        // Save expenses to file
        db.save_expenses_to_file(file_path).await.unwrap();

        // Clear the Database
        db.clear_expenses().await.unwrap();

        // Load expenses from file
        db.load_expenses_from_file(file_path).await.unwrap();

        // Check if expenses were loaded correctly
        let loaded_expenses = db.list_expenses().await.unwrap();
        assert_eq!(loaded_expenses.len(), 2);
        assert_eq!(loaded_expenses[0].name, "Test 1");
        assert_eq!(loaded_expenses[1].name, "Test 2");
    }

    async fn test_db_operations() {
        let db = Database::new().await.unwrap();

        // Create a new expense
        let expense = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "Test Expense",
            "food",
            100.0,
        )
        .unwrap();

        // Insert the expense
        let id = db.insert_expense(&expense).await.unwrap();

        // Retrieve the expense
        let retrieved = db.get_expense(id).await.unwrap().unwrap();
        assert_eq!(retrieved.name, "Test Expense");

        // Update the expense
        let mut updated = retrieved;
        updated.amount = 150.0;
        db.update_expense(&updated).await.unwrap();

        // Verify the update
        let after_update = db.get_expense(id).await.unwrap().unwrap();
        assert_eq!(after_update.amount, 150.0);

        // List all expenses
        let all_expenses = db.list_expenses().await.unwrap();
        assert!(!all_expenses.is_empty());

        // Delete the expense
        db.delete_expense(id).await.unwrap();

        // Verify deletion
        assert!(db.get_expense(id).await.unwrap().is_none());
    }
}
