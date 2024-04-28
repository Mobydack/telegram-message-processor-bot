use super::util;

#[derive(Debug)]
pub struct YandexAPI {
    pub token: String,
    pub catalog_id: String,
}

impl YandexAPI {
    pub fn new() -> Self {
        YandexAPI {
            token: util::get_env_with_scope!("YANDEX", "API_TOKEN"),
            catalog_id: util::get_env_with_scope!("YANDEX", "CATALOG_ID"),
        }
    }
}
