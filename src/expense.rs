use chrono::NaiveDate;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Expense {
    pub id: Option<i64>,
    pub date: String,
    pub name: String,
    pub category: String,
    pub amount: f64,
}

impl Expense {
    pub fn new(date: NaiveDate, name: &str, category: &str, amount: f64) -> Result<Self, String> {
        if amount < 0.0 {
            return Err("Amount must be positive".to_string());
        }
        Ok(Expense {
            id: None,
            date: date.format("%Y-%m-%d").to_string(),
            name: name.to_string(),
            category: category.to_string(),
            amount,
        })
    }
}
