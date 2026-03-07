# Payment Tracker

A Rust application for tracking payment cash in/out by parsing bank transaction emails.

## Features

- **Email Integration**: Connect to IMAP email servers to fetch bank transaction emails
- **Transaction Parsing**: Automatically parse transaction details from email content
- **Database Storage**: Store transactions in SQLite database
- **CLI Interface**: Easy-to-use command-line interface
- **Reporting**: Generate summaries and reports
- **Manual Entry**: Add transactions manually when needed

## Installation

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))
- SQLite development libraries

### Build from Source

```bash
git clone <repository-url>
cd payment-tracker
cargo build --release
```

The binary will be available at `target/release/payment-tracker`.

## Configuration

First, configure the application with your email and database settings:

```bash
# Configure email settings
payment-tracker config \
  --email your-email@gmail.com \
  --password your-app-password \
  --imap-server imap.gmail.com \
  --imap-port 993 \
  --database payments.db
```

**Note for Gmail users**: You need to use an "App Password" instead of your regular password. Enable 2-factor authentication and generate an app password from your Google account settings.

## Usage

### Initialize Database

```bash
payment-tracker init
```

### Fetch and Process Emails

```bash
payment-tracker fetch
```

This will connect to your email server, fetch unread emails from the last 7 days, parse any bank transactions, and store them in the database.

### List Transactions

```bash
# List all transactions
payment-tracker list

# List only income transactions
payment-tracker list --type in

# List transactions from a specific date range
payment-tracker list --from 2024-01-01 --to 2024-01-31

# List last 10 transactions
payment-tracker list --limit 10
```

### Generate Summary Reports

```bash
# Monthly summary (default)
payment-tracker summary

# Weekly summary
payment-tracker summary --period week

# Daily summary
payment-tracker summary --period day

# Yearly summary
payment-tracker summary --period year

# Summary for specific date
payment-tracker summary --date 2024-01-15
```

### Add Manual Transactions

```bash
# Add an income transaction
payment-tracker add --amount 1000.00 --description "Salary" --type in

# Add an expense transaction
payment-tracker add --amount 50.00 --description "Groceries" --type out --date 2024-01-15
```

## Email Parsing

The application uses regex patterns to parse common bank transaction email formats. It looks for:

- Transaction amounts (with currency symbols)
- Dates in various formats
- Descriptions/merchant names
- Transaction types (credited/debited)

Currently supported patterns include common formats from major banks. You may need to adjust the regex patterns in `src/email.rs` for your specific bank's email format.

## Database Schema

The SQLite database has a single table:

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

## Security Considerations

1. **Email Credentials**: Your email password is stored in plain text in the config file. Use app-specific passwords where possible.
2. **Database**: The SQLite database file is not encrypted. Consider encrypting sensitive data if needed.
3. **Config File**: The config file is stored at `~/.payment-tracker/config.toml`.

## Extending

### Adding New Email Parsers

To add support for a new bank's email format, modify the `parse_bank_transaction` function in `src/email.rs` to include new regex patterns for that bank's specific format.

### Adding Reports

Add new report types by extending the `App` struct's methods and adding corresponding CLI commands.

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.