use super::types::ModelAPI;

pub struct YandexModel {
    pub api_token: String,
    pub catalog_id: String,
}

impl YandexModel {
    pub fn new(api_token: String, catalog_id: String) -> Self {
        YandexModel {
            api_token,
            catalog_id,
        }
    }
}

impl ModelAPI for YandexModel {
    fn get_alternative(
        &self,
        _system_msg: String,
        _user_msg: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from("Hello, world!!!"))
    }
}
