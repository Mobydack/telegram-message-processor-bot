use super::types::ModelAPI;

#[derive(Debug)]
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

#[async_trait::async_trait]
impl ModelAPI for YandexModel {
    async fn get_alternative(
        &self,
        system_msg: String,
        user_msg: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("system_msg: {system_msg}, user_msg: {user_msg}"))
    }
}
