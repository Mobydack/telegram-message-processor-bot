use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use super::types::ModelAPI;
use super::util;
use yandex_cloud_grpc_client::yandex::cloud::ai::foundation_models::v1::{
    message::Content, text_generation_async_service_client::TextGenerationAsyncServiceClient,
    CompletionOptions, CompletionRequest, CompletionResponse, Message,
};
use yandex_cloud_grpc_client::yandex::cloud::operation::{
    operation, operation_service_client::OperationServiceClient, GetOperationRequest, Operation,
};
use yandex_cloud_grpc_client::{tonic_exports, YandexAuthApiKeyInterceptor};

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

    async fn operation_handler(
        &self,
        operation_id: &String,
        client_ref: Rc<
            RefCell<
                OperationServiceClient<
                    tonic_exports::InterceptedService<
                        tonic_exports::Channel,
                        YandexAuthApiKeyInterceptor,
                    >,
                >,
            >,
        >,
    ) -> Result<tonic_exports::prost_types::Any, tonic_exports::Status> {
        let operation_response = client_ref
            .borrow_mut()
            .get(GetOperationRequest {
                operation_id: operation_id.deref().to_string(),
            })
            .await?
            .into_inner();

        match operation_response
            .done
            .then(|| operation_response.result)
            .ok_or(tonic_exports::Status::ok("Operation isn't ready."))?
            .ok_or(tonic_exports::Status::aborted(
                "The operation is completed, but the answer is not defined.",
            ))? {
            operation::Result::Response(response) => Ok(response),
            operation::Result::Error(err) => Err(tonic_exports::Status::aborted(err.message)),
        }
    }

    async fn await_long_operation(
        &self,
        operation_id: String,
    ) -> Result<tonic_exports::prost_types::Any, tonic_exports::Status> {
        let operation_client = Rc::new(RefCell::new(OperationServiceClient::with_interceptor(
            tonic_exports::Endpoint::from_static(
                "https://llm.api.cloud.yandex.net/foundationModels/v1/completionAsync",
            )
            .connect()
            .await
                .map_err(|err| { tonic_exports::Status::aborted(err.to_string()) })?,
            YandexAuthApiKeyInterceptor {
                api_token: self.api_token.clone(),
            },
        )));

        let mut retry_configuration = util::RetryConfiguration::new(10, Duration::from_millis(500));

        util::retry_grpc_request(
            move || {
                let operation_id_clone = operation_id.clone();
                let operation_client_clone = operation_client.clone();

                async move {
                    self.operation_handler(&operation_id_clone, operation_client_clone)
                        .await
                }
            },
            &mut retry_configuration,
        )
        .await
    }
}

#[async_trait::async_trait]
impl ModelAPI for YandexModel {
    async fn get_alternative(
        &self,
        system_msg: String,
        user_msg: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let text_generation_client = Rc::new(RefCell::new(TextGenerationAsyncServiceClient::with_interceptor(
            tonic_exports::Endpoint::from_static(
                "https://llm.api.cloud.yandex.net/foundationModels/v1/completionAsync",
            )
            .connect()
            .await?,
            YandexAuthApiKeyInterceptor {
                api_token: self.api_token.clone(),
            },
        )));

        let request = CompletionRequest {
            model_uri: String::from("yandexgpt"),
            completion_options: Some(CompletionOptions {
                stream: false,
                temperature: Some(0.0),
                max_tokens: Some(10000),
            }),
            messages: vec![
                Message {
                    role: String::from("system"),
                    content: Some(Content::Text(system_msg)),
                },
                Message {
                    role: String::from("user"),
                    content: Some(Content::Text(user_msg)),
                },
            ],
        };

        let Operation {
            id: operation_id, ..
        } = text_generation_client.borrow_mut()
            .completion(request)
            .await?
            .into_inner();

        let result = self.await_long_operation(operation_id).await?;

        Ok(String::from("system_msg:, user_msg: "))
    }
}
