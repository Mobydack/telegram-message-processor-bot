pub mod model;
pub mod telegram;
mod util;
pub mod yandex_api;

use model::ModelConfiguration;
use std::sync::Arc;
use telegram::TelegramConfiguration;

#[derive(Debug)]
pub struct Configuration {
    pub telegram: Arc<TelegramConfiguration>,
    pub model: Arc<ModelConfiguration>,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            telegram: Arc::new(TelegramConfiguration::new()),
            model: Arc::new(ModelConfiguration::parse()),
        }
    }
}
