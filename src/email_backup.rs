use anyhow::{Result, anyhow};
use chrono::{Local, NaiveDate};
use regex::Regex;
use std::env;

use crate::models::Transaction;

pub struct EmailClient;

impl EmailClient {
    pub fn new(_server: &str, _port: u16, _username: &str, password: &str) -> Result<Self> {
        // Prefer the provided password; fall back to env if empty
        let final_password = if password.trim().is_empty() {
            env::var("EMAIL_APP_PASSWORD").unwrap_or_default()
        } else {
            password.to_string()
        };

        if final_password.is_empty() {
            return Err(anyhow!("Email password not provided in config and EMAIL_APP_PASSWORD environment variable not set"));
        }

        println!("Email client configured for: {}", _username);
        println!("Password: [PROTECTED] ({} characters)", final_password.len());
        
        Ok(Self {})
    }
    
    pub async fn fetch_recent_emails(&self) -> Result<Vec<String>> {
        // Simplified: In a real implementation, this would connect to IMAP
        // For now, return empty vector or mock data
        println!("Note: Email fetching is simplified in this version.");
        println!("To implement full IMAP support, you would need to:");
        println!("1. Use a stable version of the 'imap' crate");
        println!("2. Implement proper IMAP connection handling");
        println!("3. Parse email bodies with 'mailparse' crate");
        Ok(Vec::new())
    }
}

pub fn parse_transaction_from_email(email_text: &str) -> Option<Transaction> {
    // Simple regex patterns for common bank email formats
    let amount_pattern = r"(?i)amount\s*[:=]\s*[$€£]?\s*([\d,]+\.?\d*)";
    let date_pattern = r"(?i)date\s*[:=]\s*(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})";
    let description_pattern = r"(?i)description\s*[:=]\s*(.+?)(?:\n|$)";
    
    let amount_re = Regex::new(amount_pattern).ok()?;
    let date_re = Regex::new(date_pattern).ok()?;
    let desc_re = Regex::new(description_pattern).ok()?;
    
    // Extract amount
    let amount_caps = amount_re.captures(email_text)?;
    let amount_str = amount_caps.get(1)?.as_str().replace(',', "");
    let amount: f64 = amount_str.parse().ok()?;
    
    // Determine transaction type based on keywords
    let r#type = if email_text.to_lowercase().contains("credited") || 
                   email_text.to_lowercase().contains("deposited") {
        "in".to_string()
    } else if email_text.to_lowercase().contains("debited") || 
              email_text.to_lowercase().contains("withdrawn") {
        "out".to_string()
    } else {
        // Default to "out" for safety
        "out".to_string()
    };
    
    // Extract date
    let date = if let Some(date_caps) = date_re.captures(email_text) {
        if let Some(date_str) = date_caps.get(1) {
            // Try different date formats
            NaiveDate::parse_from_str(date_str.as_str(), "%d/%m/%Y")
                .or_else(|_| NaiveDate::parse_from_str(date_str.as_str(), "%m/%d/%Y"))
                .or_else(|_| NaiveDate::parse_from_str(date_str.as_str(), "%d-%m-%Y"))
                .unwrap_or_else(|_| Local::now().date_naive())
        } else {
            Local::now().date_naive()
        }
    } else {
        Local::now().date_naive()
    };
    
    // Extract description
    let description = if let Some(desc_caps) = desc_re.captures(email_text) {
        if let Some(desc) = desc_caps.get(1) {
            desc.as_str().trim().to_string()
        } else {
            if r#type == "in" {
                "Bank Credit".to_string()
            } else {
                "Bank Debit".to_string()
            }
        }
    } else {
        if r#type == "in" {
            "Bank Credit".to_string()
        } else {
            "Bank Debit".to_string()
        }
    };
    
    Some(Transaction::from_email(date, description, amount, r#type))
}

// Example email for testing
pub fn get_example_bank_email() -> String {
    r#"Subject: Transaction Alert - Debit
From: yourbank@example.com
Date: 15/01/2024

Dear Customer,

A transaction has been made on your account:

Amount: $50.00
Description: GROCERY STORE PURCHASE
Date: 15/01/2024
Transaction Type: DEBITED

Thank you,
Your Bank"#.to_string()
}