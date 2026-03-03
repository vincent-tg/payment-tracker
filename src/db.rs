use anyhow::Result;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use std::str::FromStr;

use crate::models::{Summary, Transaction};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .connect(&format!("sqlite://{}", db_path))
            .await?;
        
        Ok(Self { pool })
    }
    
    pub async fn init_database(db_path: &str) -> Result<()> {
        let pool = SqlitePoolOptions::new()
            .connect(&format!("sqlite://{}", db_path))
            .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date DATE NOT NULL,
                description TEXT NOT NULL,
                amount REAL NOT NULL,
                type TEXT NOT NULL CHECK (type IN ('in', 'out')),
                source TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(date, description, amount, type)
            )
            "#,
        )
        .execute(&pool)
        .await?;
        
        println!("Database initialized successfully at: {}", db_path);
        Ok(())
    }
    
    pub async fn insert_transaction(&self, transaction: &Transaction) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT OR IGNORE INTO transactions (date, description, amount, type, source, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(transaction.date.to_string())
        .bind(&transaction.description)
        .bind(transaction.amount)
        .bind(&transaction.r#type)
        .bind(&transaction.source)
        .bind(transaction.created_at.to_string())
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
    
    pub async fn transaction_exists(&self, transaction: &Transaction) -> Result<bool> {
        let result = sqlx::query(
            r#"
            SELECT COUNT(*) as count FROM transactions
            WHERE date = ? AND description = ? AND amount = ? AND type = ?
            "#,
        )
        .bind(transaction.date.to_string())
        .bind(&transaction.description)
        .bind(transaction.amount)
        .bind(&transaction.r#type)
        .fetch_one(&self.pool)
        .await?;
        
        let count: i64 = result.get("count");
        Ok(count > 0)
    }
    
    pub async fn get_transactions(
        &self,
        r#type: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Transaction>> {
        let mut query = "SELECT * FROM transactions WHERE 1=1".to_string();
        let mut params: Vec<String> = Vec::new();
        
        if let Some(r#type) = r#type {
            query.push_str(" AND type = ?");
            params.push(r#type.to_string());
        }
        
        if let Some(from) = from {
            query.push_str(" AND date >= ?");
            params.push(from.to_string());
        }
        
        if let Some(to) = to {
            query.push_str(" AND date <= ?");
            params.push(to.to_string());
        }
        
        query.push_str(" ORDER BY date DESC, created_at DESC");
        
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        let mut query_builder = sqlx::query_as::<_, TransactionRow>(&query);
        
        for param in params {
            query_builder = query_builder.bind(param);
        }
        
        let rows = query_builder.fetch_all(&self.pool).await?;
        
        let transactions = rows
            .into_iter()
            .map(|row| Transaction {
                id: row.id,
                date: NaiveDate::from_str(&row.date).unwrap_or_else(|_| Local::now().date_naive()),
                description: row.description,
                amount: row.amount,
                r#type: row.r#type,
                source: row.source,
                created_at: DateTime::from_str(&row.created_at).unwrap_or(Local::now()),
            })
            .collect();
        
        Ok(transactions)
    }
    
    pub async fn get_summary(&self, period: &str, date: Option<&str>) -> Result<Summary> {
        let base_date = if let Some(date_str) = date {
            NaiveDate::from_str(date_str)?
        } else {
            Local::now().date_naive()
        };
        
        let (start_date, end_date) = match period {
            "day" => (base_date, base_date),
            "week" => {
                let weekday_num = base_date.weekday().num_days_from_monday() as i64;
                let start = base_date - chrono::Duration::days(weekday_num);
                let end = start + chrono::Duration::days(6);
                (start, end)
            }
            "month" => {
                let year = base_date.year();
                let month = base_date.month();
                let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                let end = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - chrono::Duration::days(1)
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - chrono::Duration::days(1)
                };
                (start, end)
            }
            "year" => {
                let year = base_date.year();
                let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
                let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
                (start, end)
            }
            _ => (base_date, base_date),
        };
        
        // Get total transactions
        let total_result = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_count,
                COALESCE(SUM(CASE WHEN type = 'in' THEN amount ELSE 0 END), 0) as total_in,
                COALESCE(SUM(CASE WHEN type = 'out' THEN amount ELSE 0 END), 0) as total_out
            FROM transactions
            WHERE date BETWEEN ? AND ?
            "#,
        )
        .bind(start_date.to_string())
        .bind(end_date.to_string())
        .fetch_one(&self.pool)
        .await?;
        
        let total_transactions: i64 = total_result.get("total_count");
        let total_in: f64 = total_result.get("total_in");
        let total_out: f64 = total_result.get("total_out");
        let net_balance = total_in - total_out;
        
        // Get top categories (simplified - using first word of description)
        let category_result = sqlx::query(
            r#"
            SELECT 
                SUBSTR(description, 1, INSTR(description || ' ', ' ') - 1) as category,
                SUM(amount) as amount
            FROM transactions
            WHERE date BETWEEN ? AND ? AND type = 'out'
            GROUP BY category
            ORDER BY amount DESC
            LIMIT 5
            "#,
        )
        .bind(start_date.to_string())
        .bind(end_date.to_string())
        .fetch_all(&self.pool)
        .await?;
        
        let top_categories = category_result
            .iter()
            .map(|row| {
                let category: String = row.get("category");
                let amount: f64 = row.get("amount");
                (category, amount)
            })
            .collect();
        
        Ok(Summary {
            total_transactions,
            total_in,
            total_out,
            net_balance,
            top_categories,
        })
    }
}

#[derive(sqlx::FromRow)]
struct TransactionRow {
    id: i64,
    date: String,
    description: String,
    amount: f64,
    r#type: String,
    source: String,
    created_at: String,
}