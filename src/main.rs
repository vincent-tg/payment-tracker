use anyhow::Result;
use clap::{Parser, Subcommand};
use payment_tracker::{App, config::Config, db};

#[derive(Parser)]
#[command(name = "payment-tracker")]
#[command(about = "Track payment cash in/out from bank emails", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the application (email, database, etc.)
    Config {
        /// Email address to monitor
        #[arg(short, long)]
        email: Option<String>,

        /// Email password or app password
        #[arg(short, long)]
        password: Option<String>,

        /// IMAP server
        #[arg(long)]
        imap_server: Option<String>,

        /// IMAP port
        #[arg(long)]
        imap_port: Option<u16>,

        /// Database path
        #[arg(short, long)]
        database: Option<String>,
    },

    /// Fetch and process new emails
    Fetch,

    /// List transactions
    List {
        /// Filter by transaction type (in/out)
        #[arg(short, long)]
        r#type: Option<String>,

        /// Filter by date range (format: YYYY-MM-DD)
        #[arg(short, long)]
        from: Option<String>,

        /// Filter by date range (format: YYYY-MM-DD)
        #[arg(short = 'T', long)]
        to: Option<String>,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i64>,
    },

    /// Generate summary report
    Summary {
        /// Period for summary (day, week, month, year)
        #[arg(short, long, default_value = "month")]
        period: String,

        /// Specific date for summary (format: YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
    },

    /// Add a manual transaction
    Add {
        /// Transaction amount
        #[arg(short, long)]
        amount: f64,

        /// Transaction description
        #[arg(short, long)]
        description: String,

        /// Transaction type (in/out)
        #[arg(short, long)]
        r#type: String,

        /// Transaction date (format: YYYY-MM-DD, defaults to today)
        #[arg(short = 'D', long)]
        date: Option<String>,
    },

    /// Initialize the database
    Init,

    /// Start web server with health endpoints
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },

    /// Run daily VIB bank tracking
    Daily,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config {
            email,
            password,
            imap_server,
            imap_port,
            database,
        } => {
            let mut config = Config::load().unwrap_or_default();

            if let Some(email) = email {
                config.email.address = email;
            }

            if let Some(password) = password {
                config.email.password = password;
            }

            if let Some(imap_server) = imap_server {
                config.email.imap_server = imap_server;
            }

            if let Some(imap_port) = imap_port {
                config.email.imap_port = imap_port;
            }

            if let Some(database) = database {
                config.database.path = database;
            }

            config.save()?;
            println!("Configuration saved successfully!");
        }

        Commands::Fetch => {
            let config = Config::load()?;
            let app = App::new(config).await?;
            app.fetch_and_process_emails().await?;
        }

        Commands::List {
            r#type,
            from,
            to,
            limit,
        } => {
            let config = Config::load()?;
            let app = App::new(config).await?;
            app.list_transactions(r#type, from, to, limit).await?;
        }

        Commands::Summary { period, date } => {
            let config = Config::load()?;
            let app = App::new(config).await?;
            app.generate_summary(&period, date.as_deref()).await?;
        }

        Commands::Add {
            amount,
            description,
            r#type,
            date,
        } => {
            let config = Config::load()?;
            let app = App::new(config).await?;
            app.add_manual_transaction(amount, &description, &r#type, date.as_deref())
                .await?;
        }

        Commands::Init => {
            let config = Config::load()?;
            db::Database::init_database(&config.database.get_connection_string()).await?;
            println!("Database initialized successfully!");
        }

        Commands::Serve { port } => {
            println!(
                "🚀 Starting VIB Payment Tracker web server on port {}",
                port
            );
            println!("   Health endpoint: http://0.0.0.0:{}/health", port);
            payment_tracker::web::start_health_server(port).await?;
        }

        Commands::Daily => {
            println!("📅 Running daily VIB bank tracking...");
            let config = Config::load()?;
            let app = App::new(config).await?;

            // This would run the daily tracking logic
            // For now, just fetch and process emails
            app.fetch_and_process_emails().await?;
            println!("✅ Daily tracking completed!");
        }
    }

    Ok(())
}
