use super::{util, yandex_api};

#[derive(Debug, Clone)]
pub enum ModelConfiguration {
    Yandex(yandex_api::YandexAPIConfiguration),
}

impl ModelConfiguration {
    pub fn parse() -> Self {
        let model_type = util::get_env_with_scope!("MODEL", "TYPE");

        match model_type.as_str() {
            "yandex" => ModelConfiguration::Yandex(yandex_api::YandexAPIConfiguration::new()),
            _ => panic!("Invalid model type: [{model_type}]!"),
        }
    }
}
