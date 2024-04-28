pub mod model;
pub mod telegram;
mod util;
pub mod yandex_api;

use model::ModelConfiguration;
use telegram::Telegram;

#[derive(Debug)]
pub struct Configuration {
    pub telegram: Telegram,
    pub model: ModelConfiguration,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            telegram: Telegram::new(),
            model: ModelConfiguration::parse(),
        }
    }
}
