pub trait ModelAPI {
    fn get_alternative(
        &self,
        system_msg: String,
        user_msg: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
