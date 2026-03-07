# VIB Payment Tracker

A Rust application for tracking payment cash in/out by parsing bank emails, with a focus on VIB (Vietnam International Bank) transactions.

## Features

- **Email Parsing**: Automatically fetch and parse transaction emails from VIB bank
- **Multi-Currency Support**: Track transactions in VND, USD, and other currencies
- **PostgreSQL Storage**: Store transactions in a PostgreSQL database
- **REST API**: Health endpoints and transaction API via Axum web server
- **CLI Interface**: Command-line interface for manual operations
- **CI/CD Pipeline**: Automated testing, Docker builds, and k3s deployment
- **Daily Tracking**: Scheduled daily email processing

## Architecture

```
payment-tracker/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports
│   ├── config.rs          # Configuration management
│   ├── currency.rs        # Currency conversion and formatting
│   ├── db.rs              # PostgreSQL database operations
│   ├── email.rs           # Email fetching and parsing
│   ├── models.rs          # Data structures (Transaction, etc.)
│   └── web.rs             # Web server and health endpoints
├── examples/bin/          # Example programs
├── tests/                 # Unit and integration tests
├── docs/                  # Documentation
├── scripts/               # Deployment and utility scripts
├── configs/               # Configuration files
├── emails/                # Sample email files
├── k8s/                   # Kubernetes manifests
└── .github/workflows/     # GitHub Actions CI/CD
```

## Quick Start

### Prerequisites

- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- PostgreSQL 14+
- Docker (for containerization)

### Installation

```bash
# Clone the repository
git clone https://github.com/vincent-tg/payment-tracker.git
cd payment-tracker

# Build the application
cargo build --release

# Copy example configuration
cp configs/.env.example configs/.env
cp configs/config_example.toml config.toml

# Edit configuration files with your settings
# - config.toml: Database and email settings
# - configs/.env: Email app password
```

### Configuration

1. **Email Setup**:
   - Enable IMAP access in your email provider
   - Generate an app password (not your regular password)
   - Update `config.toml` with your email settings

2. **Database Setup**:
   ```bash
   # Initialize PostgreSQL database
   cargo run -- init
   ```

### Usage

```bash
# Fetch and process new emails
cargo run -- fetch

# List transactions
cargo run -- list

# Add manual transaction
cargo run -- add --amount 100.0 --description "Coffee" --type out

# Generate summary report
cargo run -- summary --period month

# Start web server
cargo run -- serve --port 8080

# Run daily tracking
cargo run -- daily
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test --lib

# Run with verbose output
cargo test -- --nocapture
```

### Building Examples

```bash
# Build all examples
cargo build --examples

# Run specific example
cargo run --example daily_vib_tracker
```

## Deployment

### Docker

```bash
# Build Docker image
docker build -t payment-tracker .

# Run container
docker run -p 8080:8080 --env-file configs/.env payment-tracker
```

### Kubernetes (k3s)

```bash
# Apply Kubernetes manifests
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -n default
```

### CI/CD Pipeline

The project includes GitHub Actions workflow for:
- Automated testing on ARM64
- Docker image building and pushing to GitHub Container Registry
- Automatic deployment to k3s cluster

## Project Structure Details

### Core Modules

- **`src/config.rs`**: Configuration management with TOML files
- **`src/currency.rs`**: Currency conversion and formatting utilities
- **`src/db.rs`**: PostgreSQL operations with SQLx
- **`src/email.rs`**: IMAP email fetching and transaction parsing
- **`src/models.rs`**: Data models with serialization support
- **`src/web.rs`**: Axum web server with health endpoints

### Email Parsing

The application specializes in parsing VIB bank emails with support for:
- VND currency formatting
- Vietnamese language transaction descriptions
- Multi-part MIME email parsing
- Base64 and quoted-printable decoding

### Database Schema

```sql
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    description TEXT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    type VARCHAR(10) NOT NULL,  -- 'in' or 'out'
    source VARCHAR(20) NOT NULL, -- 'email' or 'manual'
    bank VARCHAR(50) NOT NULL,
    transaction_id VARCHAR(100),
    email_message_id VARCHAR(100),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust for performance and safety
- Uses SQLx for type-safe SQL queries
- Axum for async web server
- Chrono for date/time handling
- Clap for command-line parsing