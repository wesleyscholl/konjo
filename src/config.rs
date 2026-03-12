use std::{env, net::SocketAddr, path::PathBuf};

use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub log_filter: String,
    pub github: GitHubConfig,
    pub workspace_root: PathBuf,
}

#[derive(Clone, Debug)]
pub struct GitHubConfig {
    pub webhook_secret: String,
    pub app_id: u64,
    pub installation_id: Option<u64>,
    pub private_key_pem: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let bind_addr = read_env_or("KONJO_BIND_ADDR", "0.0.0.0:3000")?
            .parse()
            .map_err(|source| AppError::Config(format!("invalid KONJO_BIND_ADDR: {source}")))?;

        let log_filter = read_env_or("KONJO_LOG", "info,konjo=debug")?;
        let workspace_root = match env::var("KONJO_WORKSPACE_ROOT") {
            Ok(value) => PathBuf::from(value),
            Err(_) => env::temp_dir().join("konjo"),
        };

        Ok(Self {
            bind_addr,
            log_filter,
            github: GitHubConfig {
                webhook_secret: require_env("GITHUB_WEBHOOK_SECRET")?,
                app_id: require_env("GITHUB_APP_ID")?.parse().map_err(|source| {
                    AppError::Config(format!("invalid GITHUB_APP_ID: {source}"))
                })?,
                installation_id: env::var("GITHUB_INSTALLATION_ID")
                    .ok()
                    .map(|value| {
                        value.parse().map_err(|source| {
                            AppError::Config(format!("invalid GITHUB_INSTALLATION_ID: {source}"))
                        })
                    })
                    .transpose()?,
                private_key_pem: require_env("GITHUB_PRIVATE_KEY_PEM")?,
            },
            workspace_root,
        })
    }
}

fn require_env(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|_| AppError::Config(format!("missing required env var {key}")))
}

fn read_env_or(key: &str, default: &str) -> Result<String, AppError> {
    Ok(env::var(key).unwrap_or_else(|_| default.to_owned()))
}