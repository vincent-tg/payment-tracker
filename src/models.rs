use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
    pub r#type: String, // "in" or "out"
    pub source: String, // "email" or "manual"
    pub created_at: DateTime<Local>,
}

impl Transaction {
    pub fn new_manual(amount: f64, description: &str, r#type: &str, date: Option<&str>) -> Self {
        let date = if let Some(date_str) = date {
            NaiveDate::from_str(date_str).unwrap_or_else(|_| Local::now().date_naive())
        } else {
            Local::now().date_naive()
        };
        
        Self {
            id: 0,
            date,
            description: description.to_string(),
            amount,
            r#type: r#type.to_lowercase(),
            source: "manual".to_string(),
            created_at: Local::now(),
        }
    }
    
    pub fn from_email(date: NaiveDate, description: String, amount: f64, r#type: String) -> Self {
        Self {
            id: 0,
            date,
            description,
            amount,
            r#type: r#type.to_lowercase(),
            source: "email".to_string(),
            created_at: Local::now(),
        }
    }
    
    pub fn is_in(&self) -> bool {
        self.r#type == "in"
    }
    
    pub fn is_out(&self) -> bool {
        self.r#type == "out"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub total_transactions: i64,
    pub total_in: f64,
    pub total_out: f64,
    pub net_balance: f64,
    pub top_categories: Vec<(String, f64)>,
}