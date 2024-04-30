use super::util;

#[derive(Debug, Clone)]
pub struct TelegramConfiguration {
    pub token: String,
}

impl TelegramConfiguration {
    pub fn new() -> Self {
        TelegramConfiguration {
            token: util::get_env_with_scope!("TELEGRAM", "TOKEN"),
        }
    }
}
