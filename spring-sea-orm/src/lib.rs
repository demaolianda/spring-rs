pub mod config;

use anyhow::Context;
use config::SeaOrmConfig;
use sea_orm::{ConnectOptions, Database};
use spring_boot::async_trait;
use spring_boot::config::Configurable;
use spring_boot::{app::AppBuilder, error::Result, plugin::Plugin};
use std::time::Duration;

pub type DbConn = sea_orm::DbConn;

#[derive(Configurable)]
#[config_prefix = "sea-orm"]
pub struct SeaOrmPlugin;

#[async_trait]
impl Plugin for SeaOrmPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let config = app
            .get_config::<SeaOrmConfig>(self)
            .expect("sea-orm plugin config load failed");

        let conn = Self::connect(&config)
            .await
            .expect("sea-orm plugin load failed");
        app.add_component(conn);
    }
}

impl SeaOrmPlugin {
    pub async fn connect(config: &config::SeaOrmConfig) -> Result<DbConn> {
        let mut opt = ConnectOptions::new(&config.uri);
        opt.max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .sqlx_logging(config.enable_logging);

        if let Some(connect_timeout) = config.connect_timeout {
            opt.connect_timeout(Duration::from_millis(connect_timeout));
        }
        if let Some(idle_timeout) = config.idle_timeout {
            opt.idle_timeout(Duration::from_millis(idle_timeout));
        }
        if let Some(acquire_timeout) = config.acquire_timeout {
            opt.acquire_timeout(Duration::from_millis(acquire_timeout));
        }

        Ok(Database::connect(opt)
            .await
            .with_context(|| format!("sea-orm connection failed:{}", &config.uri))?)
    }
}
