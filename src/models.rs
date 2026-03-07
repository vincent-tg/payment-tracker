use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
    pub currency: String, // "USD", "VND", "EUR", etc.
    pub r#type: String, // "in" or "out"
    pub source: String, // "email" or "manual"
    pub bank: String, // "VIB", "Chase", "PayPal", etc.
    pub transaction_id: Option<String>, // Unique ID from bank for upsert
    pub email_message_id: Option<String>, // Email message ID for tracking
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
            currency: "USD".to_string(), // Default to USD for manual entries
            r#type: r#type.to_lowercase(),
            source: "manual".to_string(),
            bank: "Manual".to_string(),
            transaction_id: None,
            email_message_id: None,
            created_at: Local::now(),
        }
    }
    
    #[allow(clippy::too_many_arguments)]
    pub fn from_email(date: NaiveDate, description: String, amount: f64, currency: String, r#type: String, bank: String, transaction_id: Option<String>, email_message_id: Option<String>) -> Self {
        Self {
            id: 0,
            date,
            description,
            amount,
            currency,
            r#type: r#type.to_lowercase(),
            source: "email".to_string(),
            bank,
            transaction_id,
            email_message_id,
            created_at: Local::now(),
        }
    }
    
    pub fn is_in(&self) -> bool {
        self.r#type == "in"
    }
    
    pub fn is_out(&self) -> bool {
        self.r#type == "out"
    }
    
    pub fn to_usd(&self) -> f64 {
        // Simple conversion for now
        match self.currency.to_uppercase().as_str() {
            "VND" => self.amount / 23000.0,
            "EUR" => self.amount / 0.92,
            "GBP" => self.amount / 0.79,
            "JPY" => self.amount / 150.0,
            _ => self.amount, // Assume USD or 1:1 for other currencies
        }
    }
    
    pub fn format_amount(&self) -> String {
        match self.currency.to_uppercase().as_str() {
            "VND" => format!("{:.0} VND", self.amount),
            "JPY" => format!("{:.0} JPY", self.amount),
            _ => format!("${:.2} {}", self.amount, self.currency),
        }
    }
    
    pub fn format_with_conversion(&self) -> String {
        if self.currency.to_uppercase() == "USD" {
            self.format_amount()
        } else {
            let usd_amount = self.to_usd();
            format!("{} (≈${:.2} USD)", self.format_amount(), usd_amount)
        }
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