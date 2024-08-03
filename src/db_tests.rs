#[cfg(test)]
mod tests {
    use crate::db::Database;
    use crate::expense::Expense;
    use chrono::NaiveDate;

    async fn setup() -> Database {
        Database::new().await.unwrap()
    }

    #[tokio::test]
    async fn test_insert_and_get_expense() {
        let db = setup().await;
        let expense = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "Test Expense",
            "Food",
            50.0,
        )
        .unwrap();

        let id = db.insert_expense(&expense).await.unwrap();
        let retrieved = db.get_expense(id).await.unwrap().unwrap();

        assert_eq!(retrieved.name, "Test Expense");
        assert_eq!(retrieved.category, "Food");
        assert_eq!(retrieved.amount, 50.0);
        assert_eq!(retrieved.date, "2023-07-28");
    }

    #[tokio::test]
    async fn test_update_expense() {
        let db = setup().await;
        let mut expense = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "Initial Expense",
            "Food",
            50.0,
        )
        .unwrap();

        let id = db.insert_expense(&expense).await.unwrap();
        expense.id = Some(id);
        expense.name = "Updated Expense".to_string();
        expense.amount = 75.0;

        db.update_expense(&expense).await.unwrap();
        let updated = db.get_expense(id).await.unwrap().unwrap();

        assert_eq!(updated.name, "Updated Expense");
        assert_eq!(updated.amount, 75.0);
    }

    #[tokio::test]
    async fn test_delete_expense() {
        let db = setup().await;
        let expense = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "To Be Deleted",
            "Food",
            50.0,
        )
        .unwrap();

        let id = db.insert_expense(&expense).await.unwrap();
        db.delete_expense(id).await.unwrap();

        let result = db.get_expense(id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_expenses() {
        let db = setup().await;
        let expense1 = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "Expense 1",
            "Food",
            50.0,
        )
        .unwrap();
        let expense2 = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 29).unwrap(),
            "Expense 2",
            "Transport",
            30.0,
        )
        .unwrap();

        db.insert_expense(&expense1).await.unwrap();
        db.insert_expense(&expense2).await.unwrap();

        let expenses = db.list_expenses().await.unwrap();
        assert_eq!(expenses.len(), 2);
        assert!(expenses
            .iter()
            .any(|e| e.name == "Expense 1" && e.category == "Food"));
        assert!(expenses
            .iter()
            .any(|e| e.name == "Expense 2" && e.category == "Transport"));
    }

    #[tokio::test]
    async fn test_filter_by_category() {
        let db = setup().await;
        let expense1 = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "Expense 1",
            "Food",
            50.0,
        )
        .unwrap();
        let expense2 = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 29).unwrap(),
            "Expense 2",
            "Transport",
            30.0,
        )
        .unwrap();

        db.insert_expense(&expense1).await.unwrap();
        db.insert_expense(&expense2).await.unwrap();

        let food_expenses = db.filter_by_category("Food").await.unwrap();
        assert_eq!(food_expenses.len(), 1);
        assert_eq!(food_expenses[0].name, "Expense 1");

        let transport_expenses = db.filter_by_category("Transport").await.unwrap();
        assert_eq!(transport_expenses.len(), 1);
        assert_eq!(transport_expenses[0].name, "Expense 2");
    }

    #[tokio::test]
    async fn test_filter_by_month() {
        let db = setup().await;
        let expense1 = Expense::new(
            NaiveDate::from_ymd_opt(2023, 7, 28).unwrap(),
            "July Expense",
            "Food",
            50.0,
        )
        .unwrap();
        let expense2 = Expense::new(
            NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
            "August Expense",
            "Transport",
            30.0,
        )
        .unwrap();

        db.insert_expense(&expense1).await.unwrap();
        db.insert_expense(&expense2).await.unwrap();

        let july_expenses = db.filter_by_month(2023, 7).await.unwrap();
        assert_eq!(july_expenses.len(), 1);
        assert_eq!(july_expenses[0].name, "July Expense");

        let august_expenses = db.filter_by_month(2023, 8).await.unwrap();
        assert_eq!(august_expenses.len(), 1);
        assert_eq!(august_expenses[0].name, "August Expense");
    }
}
