// Partial copy code from https://docs.rs/yandex-cloud/2023.9.4/yandex_cloud/index.html
use tonic::{metadata::AsciiMetadataValue, service::Interceptor, Request, Status};

pub mod tonic_exports {
    pub use tonic::service::interceptor::InterceptedService;
    pub use tonic::transport::Channel;
    pub use tonic::transport::Endpoint;
    pub use tonic::{Status,Code};
    pub use prost_types;
}

pub struct YandexAuthApiKeyInterceptor {
    pub api_token: String,
}

impl Interceptor for YandexAuthApiKeyInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let token: AsciiMetadataValue = format!("Api-Key {}", &self.api_token)
            .try_into()
            .map_err(|_| {
                Status::invalid_argument("authorization token contained invalid characters")
            })?;

        request
            .metadata_mut()
            .insert("Authorization", token);

        Ok(request)
    }
}

include!("client/includes.rs");
