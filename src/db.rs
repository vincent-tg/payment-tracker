use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Datelike, Local, NaiveDate};
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use std::str::FromStr;

use crate::models::{Summary, Transaction};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .min_connections(0)
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(10))
            .idle_timeout(std::time::Duration::from_secs(60))
            .max_lifetime(std::time::Duration::from_secs(300))
            .test_before_acquire(true)
            .connect(connection_string)
            .await?;

        Ok(Self { pool })
    }

    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn init_database(connection_string: &str) -> Result<()> {
        let pool = PgPoolOptions::new().connect(connection_string).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id SERIAL PRIMARY KEY,
                date DATE NOT NULL,
                description TEXT NOT NULL,
                amount DOUBLE PRECISION NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                type TEXT NOT NULL CHECK (type IN ('in', 'out')),
                source TEXT NOT NULL,
                bank TEXT NOT NULL DEFAULT 'Unknown',
                transaction_id TEXT,
                email_message_id TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(transaction_id, bank),
                UNIQUE(email_message_id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        println!(
            "Database initialized successfully with connection: {}",
            connection_string
        );
        Ok(())
    }

    pub async fn insert_transaction(&self, transaction: &Transaction) -> Result<i64> {
        // Use transaction_id for upsert if available, otherwise use email_message_id
        let conflict_target = if transaction.transaction_id.is_some() {
            "(transaction_id, bank)"
        } else if transaction.email_message_id.is_some() {
            "(email_message_id)"
        } else {
            // Fallback to composite key
            "(date, description, amount, type, currency, bank)"
        };

        let query = format!(
            r#"
            INSERT INTO transactions (date, description, amount, currency, type, source, bank, transaction_id, email_message_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT {conflict_target} 
            DO UPDATE SET 
                date = EXCLUDED.date,
                description = EXCLUDED.description,
                amount = EXCLUDED.amount,
                currency = EXCLUDED.currency,
                type = EXCLUDED.type,
                source = EXCLUDED.source,
                created_at = EXCLUDED.created_at
            RETURNING id
            "#
        );

        let result = sqlx::query(&query)
            .bind(transaction.date)
            .bind(&transaction.description)
            .bind(transaction.amount)
            .bind(&transaction.currency)
            .bind(&transaction.r#type)
            .bind(&transaction.source)
            .bind(&transaction.bank)
            .bind(transaction.transaction_id.as_deref())
            .bind(transaction.email_message_id.as_deref())
            .bind(transaction.created_at.naive_utc())
            .fetch_one(&self.pool)
            .await?;

        let id: i64 = result.get("id");
        Ok(id)
    }

    pub async fn transaction_exists(&self, transaction: &Transaction) -> Result<bool> {
        // First check by transaction_id if available
        if let Some(ref transaction_id) = transaction.transaction_id {
            let result = sqlx::query(
                r#"
                SELECT COUNT(*) as count FROM transactions
                WHERE transaction_id = $1 AND bank = $2
                "#,
            )
            .bind(transaction_id)
            .bind(&transaction.bank)
            .fetch_one(&self.pool)
            .await?;

            let count: i64 = result.get("count");
            if count > 0 {
                return Ok(true);
            }
        }

        // Then check by email_message_id if available
        if let Some(ref email_message_id) = transaction.email_message_id {
            let result = sqlx::query(
                r#"
                SELECT COUNT(*) as count FROM transactions
                WHERE email_message_id = $1
                "#,
            )
            .bind(email_message_id)
            .fetch_one(&self.pool)
            .await?;

            let count: i64 = result.get("count");
            if count > 0 {
                return Ok(true);
            }
        }

        // Fallback to composite key check
        let result = sqlx::query(
            r#"
            SELECT COUNT(*) as count FROM transactions
            WHERE date = $1 AND description = $2 AND amount = $3 AND type = $4 AND currency = $5 AND bank = $6
            "#,
        )
        .bind(transaction.date)
        .bind(&transaction.description)
        .bind(transaction.amount)
        .bind(&transaction.r#type)
        .bind(&transaction.currency)
        .bind(&transaction.bank)
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
        let mut param_count = 0;

        if let Some(r#type) = r#type {
            param_count += 1;
            query.push_str(&format!(" AND type = ${}", param_count));
            params.push(r#type.to_string());
        }

        if let Some(from) = from {
            param_count += 1;
            query.push_str(&format!(" AND date >= ${}", param_count));
            params.push(from.to_string());
        }

        if let Some(to) = to {
            param_count += 1;
            query.push_str(&format!(" AND date <= ${}", param_count));
            params.push(to.to_string());
        }

        query.push_str(" ORDER BY date DESC, created_at DESC");

        if let Some(limit) = limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
            params.push(limit.to_string());
        }

        let mut query_builder = sqlx::query_as::<_, TransactionRow>(&query);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        let transactions = rows
            .into_iter()
            .map(|row| {
                let date = NaiveDate::from_str(&row.date).with_context(|| {
                    format!("Invalid date in DB row id={}: {}", row.id, row.date)
                })?;
                let created_at = DateTime::from_str(&row.created_at).with_context(|| {
                    format!(
                        "Invalid created_at in DB row id={}: {}",
                        row.id, row.created_at
                    )
                })?;

                Ok(Transaction {
                    id: row.id,
                    date,
                    description: row.description,
                    amount: row.amount,
                    currency: row.currency,
                    r#type: row.r#type,
                    source: row.source,
                    bank: row.bank,
                    transaction_id: row.transaction_id,
                    email_message_id: row.email_message_id,
                    created_at,
                })
            })
            .collect::<Result<Vec<_>>>()?;

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
                let start = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| {
                    anyhow!("Invalid date: year={}, month={}, day=1", year, month)
                })?;
                let end = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1)
                        .ok_or_else(|| anyhow!("Invalid date: year={}, month=1, day=1", year + 1))?
                        - chrono::Duration::days(1)
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1).ok_or_else(|| {
                        anyhow!("Invalid date: year={}, month={}, day=1", year, month + 1)
                    })? - chrono::Duration::days(1)
                };
                (start, end)
            }
            "year" => {
                let year = base_date.year();
                let start = NaiveDate::from_ymd_opt(year, 1, 1)
                    .ok_or_else(|| anyhow!("Invalid date: year={}, month=1, day=1", year))?;
                let end = NaiveDate::from_ymd_opt(year, 12, 31)
                    .ok_or_else(|| anyhow!("Invalid date: year={}, month=12, day=31", year))?;
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
            WHERE date BETWEEN $1 AND $2
            "#,
        )
        .bind(start_date)
        .bind(end_date)
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
                SPLIT_PART(description, ' ', 1) as category,
                SUM(amount) as amount
            FROM transactions
            WHERE date BETWEEN $1 AND $2 AND type = 'out'
            GROUP BY category
            ORDER BY amount DESC
            LIMIT 5
            "#,
        )
        .bind(start_date)
        .bind(end_date)
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

    pub async fn get_transaction_by_id(&self, id: i64) -> Result<Option<Transaction>> {
        let row = sqlx::query_as::<_, TransactionRow>("SELECT * FROM transactions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| Transaction {
            id: row.id,
            date: NaiveDate::from_str(&row.date).unwrap_or_else(|_| Local::now().date_naive()),
            description: row.description,
            amount: row.amount,
            currency: row.currency,
            r#type: row.r#type,
            source: row.source,
            bank: row.bank,
            transaction_id: row.transaction_id,
            email_message_id: row.email_message_id,
            created_at: DateTime::from_str(&row.created_at).unwrap_or(Local::now()),
        }))
    }

    pub async fn delete_transaction(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(sqlx::FromRow)]
struct TransactionRow {
    id: i64,
    date: String,
    description: String,
    amount: f64,
    currency: String,
    r#type: String,
    source: String,
    bank: String,
    transaction_id: Option<String>,
    email_message_id: Option<String>,
    created_at: String,
}
