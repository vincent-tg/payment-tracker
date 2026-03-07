# Quick Start Guide for Payment Tracker

## Overview
A Rust application for tracking payment cash in/out with simplified email parsing capabilities.

## Features
- ✅ Track income and expenses
- ✅ SQLite database storage  
- ✅ CLI interface with comprehensive commands
- ✅ Transaction listing with filtering
- ✅ Summary reports
- ✅ Simplified email parsing (regex-based)
- ❌ Full IMAP email integration (simplified in this version)

## Installation

### Prerequisites
- Rust and Cargo installed
- SQLite development libraries

### Build from Source
```bash
cargo build --release
```

The binary will be at `target/release/payment-tracker`.

## Basic Usage

### 1. Initialize Database
```bash
./target/release/payment-tracker init
```

### 2. Configure (Optional)
Create `~/.payment-tracker/config.toml`:
```toml
[email]
address = "your-email@gmail.com"
password = "your-app-password"
imap_server = "imap.gmail.com"
imap_port = 993

[database]
path = "payment_tracker.db"
```

**Note for Gmail users**: Use an "App Password" (not your regular password).

### 3. Add Transactions Manually
```bash
# Add income
./target/release/payment-tracker add --amount 1000.00 --description "Salary" --type in

# Add expense  
./target/release/payment-tracker add --amount 50.00 --description "Groceries" --type out --date 2024-01-15
```

### 4. List Transactions
```bash
# List all transactions
./target/release/payment-tracker list

# List only income
./target/release/payment-tracker list --type in

# List with date range
./target/release/payment-tracker list --from 2024-01-01 --to 2024-01-31

# Limit results
./target/release/payment-tracker list --limit 5
```

### 5. Generate Reports
```bash
# Monthly summary (default)
./target/release/payment-tracker summary

# Weekly summary
./target/release/payment-tracker summary --period week

# Daily summary
./target/release/payment-tracker summary --period day

# Yearly summary
./target/release/payment-tracker summary --period year
```

## Email Parsing (Simplified)

The current version includes simplified email parsing using regex patterns. It can parse transaction information from email text that follows common bank formats.

### Example Email Format
```text
Subject: Transaction Alert
Date: 15/01/2024

Amount: $50.00
Description: GROCERY STORE PURCHASE
Date: 15/01/2024
Transaction Type: DEBITED
```

### Testing Email Parsing
```rust
// Example code to test email parsing
use payment_tracker::email;

let email_text = r#"Amount: $50.00
Description: Grocery Store
Date: 15/01/2024"#;

if let Some(transaction) = email::parse_transaction_from_email(email_text) {
    println!("Parsed transaction: {:?}", transaction);
}
```

## Database Schema
```sql
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL,
    description TEXT NOT NULL,
    amount REAL NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('in', 'out')),
    source TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(date, description, amount, type)
);
```

## Project Structure
```
payment-tracker/
├── src/
│   ├── main.rs          # CLI interface
│   ├── lib.rs           # Main application logic
│   ├── config.rs        # Configuration management
│   ├── db.rs           # Database operations
│   ├── email.rs        # Simplified email parsing
│   ├── models.rs       # Data structures
│   └── transactions.rs # Transaction utilities
├── Cargo.toml
├── README.md
└── example_usage.sh
```

## Extending the Application

### Adding Full IMAP Support
To add full IMAP email fetching:

1. Update `Cargo.toml` with stable `imap` and `mailparse` crates
2. Implement proper IMAP connection in `email.rs`
3. Update `EmailClient::fetch_recent_emails()` to connect to real IMAP server

### Adding New Features
- **Export functionality**: Add CSV/JSON export commands
- **Categories**: Enhance transaction categorization
- **Recurring transactions**: Support for scheduled transactions
- **Budget tracking**: Add budget management features

## Troubleshooting

### Common Issues

1. **Database errors**: Ensure SQLite is installed
2. **Build errors**: Check Rust version (1.70+ recommended)
3. **Permission errors**: Check write permissions for database file

### Getting Help
- Check the `README.md` for detailed documentation
- Review the example usage in `example_usage.sh`
- Examine the source code for implementation details

## License
MIT

## Contributing
Contributions are welcome! Please submit pull requests or open issues on the GitHub repository.