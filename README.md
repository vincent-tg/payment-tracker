# Payment Tracker

A production-ready Rust application for tracking payment cash in/out by parsing bank transaction emails, with a specialized focus on fetching VIB (Vietnam International Bank) activity.

## 📦 Features

- **Email Integration**: Connects to the user's IMAP email server (e.g. Gmail) to securely fetch and parse transaction notification emails.
- **Transaction Parsing**: Automatically parses transaction details using Regex (amounts, dates, descriptions, cash in/out).
- **Database Storage**: Integrates seamlessly with SQLite for local persistence or PostgreSQL (when deployed) via the `sqlx` crate.
- **CLI Interface**: Exposes a developer-friendly command-line experience with 6 subcommands (`config`, `fetch`, `list`, `summary`, `add`, `init`).
- **Reporting**: Easily generate summaries and aggregate financial reports by period (day, week, month, year).
- **Containerization**: Includes a lightweight, multi-stage Alpine Dockerfile.
- **Kubernetes / CI-CD**: Fully orchestrated k3s kubernetes manifests and an automated GitHub Actions CI/CD Pipeline.

## 📁 Repository Structure

```text
payment-tracker/
├── src/                    # Rust source code
│   ├── main.rs             # CLI interface (subcommands router)
│   ├── lib.rs              # Application core logic 
│   ├── config.rs           # Configuration state management (TOML base)
│   ├── db.rs               # Database bindings and query builders
│   ├── email.rs            # IMAP parsing and Regex logic
│   ├── models.rs           # Transaction & Summary Object models
│   └── transactions.rs     # Transaction processing utilities
├── k8s/                    # Kubernetes manifests (Deployments, PVCs, CronJobs)
├── .github/workflows/      # Automated CI/CD pipelines
├── Dockerfile              # Multi-stage Alpine container build
├── docker-compose.yml      # Local architecture deployment
├── Makefile                # Build automation
├── docs/                   # Additional deep-dive documentation (VIB patterns)
└── README.md               # Main documentation
```

## 🚀 Quick Start (Local Setup)

### Prerequisites

- Rust and Cargo
- SQLite or PostgreSQL
- Docker (optional)

### Installation

1. Clone the repository and build:
```bash
git clone https://github.com/vincent-tg/payment-tracker.git
cd payment-tracker
cargo build --release
```

2. Initialize the application config and database:
```bash
payment-tracker config \
  --email your-email@gmail.com \
  --password your-app-password \
  --imap-server imap.gmail.com \
  --imap-port 993 \
  --database payments.db

payment-tracker init
```
*(Note for Gmail users: Use an app-specific password instead of your main password)*

3. Fetch transaction emails:
```bash
payment-tracker fetch
```

4. View your tracked payments:
```bash
payment-tracker summary --period month
payment-tracker list --limit 10
```

## 🐳 Docker Production Setup 

You can easily instantiate the microservice locally using Docker Compose, circumventing the need to build the binary locally.

```bash
make docker-build
make docker-run
```

## ☸️ Kubernetes (k3s) Deployment

Production environments utilize the provided k3s/Kubernetes manifests to deploy the `payment-tracker` image securely alongside its CRON configurations and Secret resources.

1. Configure GitHub Container Registry credentials in your cluster to pull the image.
2. Ensure your environmental databases (e.g. Supabase postgres instance) are mounted via kubernetes `Secret`.
3. Apply the custom definitions:

```bash
kubectl apply -f k8s/payment-tracker.yaml -n default
```

You can view the system's runtime health logic configured out-of-the-box:
```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8080
```

## ⚙️ CI/CD Pipeline Architecture

The `.github/workflows/ci-cd.yml` maintains strict quality and reliability standards through GitHub Actions:
- **Test Phase**: Formatting, Linting (Clippy), Compiling, and Unit Tests execution.
- **Build Phase**: Mutli-arch Docker image packaging & artifact uploads via GitHub Container Registry (GHCR).
- **Deploy Phase**: Continuous, automated rollbacks & declarative deployments directly to the active k3s environments.

## License

MIT