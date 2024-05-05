mod configuration;
mod model;
mod telegram_bot;
mod api;

use crate::model::ModelAPIFactory;
use std::{env, panic};
use tokio::signal;

async fn graceful_shutdown_handler() {
    log::info!("Stopping application...");
}

fn set_default_log_level() {
    if let Err(_) = env::var("LOG_LEVEL") {
        env::set_var("LOG_LEVEL", "info");
    }
}

#[tokio::main]
async fn main() {
    set_default_log_level();

    pretty_env_logger::init_custom_env("LOG_LEVEL");

    log::info!("Starting application...");

    panic::set_hook(Box::new(|panic_info| {
        log::error!("{panic_info}");
    }));

    let app_config = configuration::Configuration::new();
    let model = ModelAPIFactory::create(&app_config.model);

    telegram_bot::create(&app_config.telegram, model).await;

    tokio::select! {
        _ = signal::ctrl_c() => { graceful_shutdown_handler().await }
    }
}
