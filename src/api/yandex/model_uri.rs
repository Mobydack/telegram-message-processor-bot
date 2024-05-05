use std::fmt;

#[derive(Debug)]
pub enum ModelUri {
    /// String - catalog_id
    YandexGPTPro(String),
}

impl fmt::Display for ModelUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}", match self {
                ModelUri::YandexGPTPro(value) => format!("gpt://{value}/yandexgpt/latest"),
            }
        )
    }
}
