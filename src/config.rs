use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

pub const PASSWORD_ENV_VAR: &str = "EMAIL_APP_PASSWORD";
pub const LEGACY_PASSWORD_ENV_VAR: &str = "EMAIL_PASSWORD";
pub const PASSWORD_PLACEHOLDER: &str = "USE_ENV_VAR";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailConfig {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub password: String,
    #[serde(default = "default_imap_server")]
    pub imap_server: String,
    #[serde(default = "default_imap_port")]
    pub imap_port: u16,
    pub provider: Option<String>,
}

fn default_imap_server() -> String {
    "imap.gmail.com".to_string()
}

fn default_imap_port() -> u16 {
    993
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub connection_string: Option<String>,
}

impl DatabaseConfig {
    pub fn get_connection_string(&self) -> String {
        if let Some(conn_str) = &self.connection_string {
            if !conn_str.trim().is_empty() {
                return conn_str.clone();
            }
        }

        if self.path.starts_with("postgres://") || self.path.starts_with("postgresql://") {
            return self.path.clone();
        }

        // Fallback to SQLite for backward compatibility
        format!("sqlite://{}", self.path)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub email: EmailConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmailProvider {
    Gmail,
    Outlook,
    Yahoo,
    #[allow(dead_code)]
    ProtonMail,
    #[allow(dead_code)]
    FastMail,
    Custom,
}

impl EmailProvider {
    pub fn from_provider_name(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "gmail" | "google" => EmailProvider::Gmail,
            "outlook" | "office365" | "microsoft" => EmailProvider::Outlook,
            "yahoo" => EmailProvider::Yahoo,
            "protonmail" | "proton" => EmailProvider::ProtonMail,
            "fastmail" => EmailProvider::FastMail,
            _ => EmailProvider::Custom,
        }
    }

    pub fn get_imap_settings(&self) -> (&str, u16) {
        match self {
            EmailProvider::Gmail => ("imap.gmail.com", 993),
            EmailProvider::Outlook => ("outlook.office365.com", 993),
            EmailProvider::Yahoo => ("imap.mail.yahoo.com", 993),
            EmailProvider::ProtonMail => ("127.0.0.1", 1143),
            EmailProvider::FastMail => ("imap.fastmail.com", 993),
            EmailProvider::Custom => ("imap.example.com", 993),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            email: EmailConfig {
                address: String::new(),
                password: String::new(),
                imap_server: "imap.gmail.com".to_string(),
                imap_port: 993,
                provider: Some("gmail".to_string()),
            },
            database: DatabaseConfig {
                path: "payment_tracker.db".to_string(),
                connection_string: None,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if Path::new(&config_path).exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("failed to read config file at {config_path}"))?;
            let mut config: Config = toml::from_str(&content)
                .with_context(|| format!("failed to parse TOML in {config_path}"))?;
            config.apply_provider_settings();

            // Override database connection string from environment if provided
            if let Ok(db_url) =
                env::var("DATABASE_URL").or_else(|_| env::var("SUPABASE_CONNECTION_STRING"))
            {
                config.database.connection_string = Some(db_url);
            }

            if let Ok(email) = env::var("EMAIL_ADDRESS") {
                config.email.address = email;
            }

            // Prefer EMAIL_APP_PASSWORD, keep EMAIL_PASSWORD for backward compatibility
            if let Ok(pass) = env::var(PASSWORD_ENV_VAR) {
                config.email.password = pass;
            } else if let Ok(pass) = env::var(LEGACY_PASSWORD_ENV_VAR) {
                config.email.password = pass;
            }

            if let Ok(server) = env::var("IMAP_SERVER") {
                config.email.imap_server = server;
            }

            if let Ok(port) = env::var("IMAP_PORT") {
                if let Ok(p) = port.parse() {
                    config.email.imap_port = p;
                }
            }

            Ok(config)
        } else {
            let mut config = Config::default();

            // Override with env var if present even for fresh config
            if let Ok(db_url) =
                env::var("DATABASE_URL").or_else(|_| env::var("SUPABASE_CONNECTION_STRING"))
            {
                config.database.connection_string = Some(db_url);
            }
            if let Ok(email) = env::var("EMAIL_ADDRESS") {
                config.email.address = email;
            }
            if let Ok(pass) = env::var(PASSWORD_ENV_VAR) {
                config.email.password = pass;
            } else if let Ok(pass) = env::var(LEGACY_PASSWORD_ENV_VAR) {
                config.email.password = pass;
            }
            if let Ok(server) = env::var("IMAP_SERVER") {
                config.email.imap_server = server;
            }
            if let Ok(port) = env::var("IMAP_PORT") {
                if let Ok(p) = port.parse() {
                    config.email.imap_port = p;
                }
            }
            config.save()?;
            Ok(config)
        }
    }

    pub fn resolved_email_password(&self) -> Option<String> {
        if let Ok(password) = env::var(PASSWORD_ENV_VAR) {
            if !password.trim().is_empty() {
                return Some(password);
            }
        }

        if let Ok(password) = env::var(LEGACY_PASSWORD_ENV_VAR) {
            if !password.trim().is_empty() {
                return Some(password);
            }
        }

        let cfg_password = self.email.password.trim();
        if cfg_password.is_empty() || cfg_password == PASSWORD_PLACEHOLDER {
            None
        } else {
            Some(self.email.password.clone())
        }
    }

    fn apply_provider_settings(&mut self) {
        if let Some(ref provider) = self.email.provider {
            let provider_type = EmailProvider::from_provider_name(provider);
            let (server, port) = provider_type.get_imap_settings();

            if self.email.imap_server.is_empty() || self.email.imap_server == "imap.example.com" {
                self.email.imap_server = server.to_string();
            }
            if self.email.imap_port == 0 || self.email.imap_port == 993 {
                self.email.imap_port = port;
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        if let Some(parent) = Path::new(&config_path).parent() {
            if !parent.exists() && parent != Path::new("") {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create config directory {}", parent.display())
                })?;
            }
        }

        let mut sanitized = self.clone();
        if !sanitized.email.password.trim().is_empty()
            && sanitized.email.password.trim() != PASSWORD_PLACEHOLDER
        {
            eprintln!(
                "⚠️  For security, email password is not stored in config.toml. Set {} in your environment.",
                PASSWORD_ENV_VAR
            );
            sanitized.email.password = PASSWORD_PLACEHOLDER.to_string();
        }

        let content = toml::to_string_pretty(&sanitized).context("failed to serialize config")?;
        fs::write(&config_path, content)
            .with_context(|| format!("failed to write config file at {config_path}"))?;
        Ok(())
    }

    fn config_path() -> String {
        if Path::new("/app/config.toml").exists() {
            return "/app/config.toml".to_string();
        }
        if Path::new("./config.toml").exists() {
            return "./config.toml".to_string();
        }

        // If there's an explicit /app directory but the file doesn't exist yet, we're likely in docker
        if Path::new("/app").exists() {
            return "/app/config.toml".to_string();
        }

        let home = match std::env::var("HOME") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => ".".to_string(),
        };
        format!("{}/.payment-tracker/config.toml", home)
    }
}
