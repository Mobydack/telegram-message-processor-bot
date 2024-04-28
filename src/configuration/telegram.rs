use super::util;

#[derive(Debug)]
pub struct Telegram {
    pub token: String,
}

impl Telegram {
    pub fn new() -> Self {
        Telegram {
            token: util::get_env_with_scope!("TELEGRAM", "TOKEN"),
        }
    }
}
