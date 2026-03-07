# Examples

This directory contains example programs demonstrating how to use the payment-tracker library.

## Available Examples

### `daily_vib_tracker.rs`
Demonstrates daily tracking of VIB bank emails. This example:
- Loads configuration from config.toml
- Connects to email server via IMAP
- Fetches recent emails
- Parses transactions from VIB bank emails
- Saves transactions to PostgreSQL database
- Generates financial summary

**Usage:**
```bash
cargo run --example daily_vib_tracker
```

### `init_postgres.rs`
Initializes PostgreSQL database and creates sample transactions. This example:
- Creates database tables if they don't exist
- Inserts sample VIB transactions
- Demonstrates database connection and operations

**Usage:**
```bash
cargo run --example init_postgres
```

### `final_demo.rs`
Comprehensive demonstration of all payment-tracker features. This example:
- Shows currency conversion between VND and USD
- Demonstrates transaction formatting
- Shows database operations
- Displays financial summaries

**Usage:**
```bash
cargo run --example final_demo
```

### `analyze_email_patterns.rs`
Analyzes email patterns for transaction extraction. This example:
- Parses sample email files
- Extracts transaction information
- Shows email parsing techniques

**Usage:**
```bash
cargo run --example analyze_email_patterns
```

### `extract_vib_transaction_id.rs`
Specialized example for extracting VIB bank transaction IDs. This example:
- Focuses on VIB-specific email formats
- Extracts unique transaction identifiers
- Demonstrates regex pattern matching

**Usage:**
```bash
cargo run --example extract_vib_transaction_id
```

## Building Examples

```bash
# Build all examples
cargo build --examples

# Build specific example
cargo build --example daily_vib_tracker

# Run specific example
cargo run --example daily_vib_tracker
```

## Example Configuration

Examples require proper configuration:
1. Copy `configs/.env.example` to `configs/.env`
2. Copy `configs/config_example.toml` to `config.toml`
3. Edit configuration files with your email and database settings

## Adding New Examples

When adding new examples:
1. Place `.rs` files in the `examples/bin/` directory
2. Add entry to `Cargo.toml` under `[[example]]` section
3. Update this README with example description
4. Ensure examples compile without errors