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
            query("INSERT INTO expenses (date, name, category, amount) VALUES (?, ?, ?, ?)")
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

    pub async fn list_expenses(&self) -> Result<Vec<Expense>, sqlx::Error> {
        query_as::<_, Expense>("SELECT id, date, name, category, amount FROM expenses")
            .fetch_all(&self.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[tokio::test]
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
