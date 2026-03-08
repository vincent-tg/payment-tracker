use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailConfig {
    pub address: String,
    pub password: String,
    pub imap_server: String,
    pub imap_port: u16,
    pub provider: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub connection_string: Option<String>,
}

impl DatabaseConfig {
    pub fn get_connection_string(&self) -> String {
        if let Some(conn_str) = &self.connection_string {
            conn_str.clone()
        } else {
            // Fallback to SQLite for backward compatibility
            format!("sqlite://{}", self.path)
        }
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
            let content = fs::read_to_string(&config_path)?;
            let mut config: Config = toml::from_str(&content)?;
            config.apply_provider_settings();
            // Override database connection string from environment if provided 
            if let Ok(db_url) = env::var("DATABASE_URL").or_else(|_| env::var("SUPABASE_CONNECTION_STRING")) {
                config.database.connection_string = Some(db_url);
            }
            if let Ok(email) = env::var("EMAIL_ADDRESS") {
                config.email.address = email;
            }
            if let Ok(pass) = env::var("EMAIL_PASSWORD") {
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
            if let Ok(db_url) = env::var("DATABASE_URL").or_else(|_| env::var("SUPABASE_CONNECTION_STRING")) {
                config.database.connection_string = Some(db_url);
            }
            if let Ok(email) = env::var("EMAIL_ADDRESS") {
                config.email.address = email;
            }
            if let Ok(pass) = env::var("EMAIL_PASSWORD") {
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
                fs::create_dir_all(parent).unwrap_or_default();
            }
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
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

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.payment-tracker/config.toml", home)
    }
}
