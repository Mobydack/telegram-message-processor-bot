use std::fmt::Debug;

#[async_trait::async_trait]
pub trait ModelAPI: Send + Sync + Debug {
    async fn get_alternative(
        &self,
        system_msg: String,
        user_msg: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
