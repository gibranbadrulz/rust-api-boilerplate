use clap::Parser;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;
use std::fmt;

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum AppEnv {
    Development,
    Production,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Development => write!(f, "Development"),
            AppEnv::Production => write!(f, "Production"),
        }
    }
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum)]
    pub app_env: AppEnv,

    #[clap(long, env, default_value = "8000")]
    pub app_port: u16,

    #[clap(long, env, default_value = "127.0.0.1")]
    pub app_host: String,

    #[clap(long, env)]
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self::parse()
    }

    pub async fn create_pg_pool(&self) -> Result<PgPool, Error> {
        PgPoolOptions::new()
            .max_connections(15)
            .connect(&self.database_url)
            .await
    }
}
