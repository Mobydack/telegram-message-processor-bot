use super::util;

#[derive(Debug, Clone)]
pub struct YandexAPIConfiguration {
    pub token: String,
    pub catalog_id: String,
}

impl YandexAPIConfiguration {
    pub fn new() -> Self {
        YandexAPIConfiguration {
            token: util::get_env_with_scope!("YANDEX", "API_TOKEN"),
            catalog_id: util::get_env_with_scope!("YANDEX", "CATALOG_ID"),
        }
    }
}
