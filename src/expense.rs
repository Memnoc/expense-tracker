use chrono::NaiveDate;

#[derive(Debug)]
pub struct Expense {
    pub date: NaiveDate,
    pub name: String,
    pub category: String,
    pub amount: f64,
}

impl Expense {
    pub fn new(date: NaiveDate, name: &str, category: &str, amount: f64) -> Result<Self, String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
        Ok(Expense {
            date,
            name: name.to_string(),
            category: category.to_string(),
            amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_create_expense() {
        let date = NaiveDate::from_ymd_opt(2023, 7, 28).unwrap();
        let expense = Expense::new(date, "Grocery shopping", "food", 50.0);
        assert!(expense.is_ok());
    }

    #[test]
    fn create_invalid_expense() {
        let date = NaiveDate::from_ymd_opt(2023, 7, 28).unwrap();
        let expense = Expense::new(date, "Grocery shopping", "food", -10.0);
        assert!(expense.is_err());
    }
}
