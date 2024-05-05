
use crate::api::util::fetch_with_retry;
use super::model_uri::ModelUri;
use reqwest::{header, Client};
use serde_json::json;
use super::types::Operation;

const AI_FOUNDATION_MODELS_ENDPOINT: &str = "https://llm.api.cloud.yandex.net";
const OPERATION_ENDPOINT: &str = "https://operation.api.cloud.yandex.net";

#[derive(Debug)]
pub struct YandexRestApi {
    catalog_id: String,
    client: Client,
}

impl YandexRestApi {
    pub fn new(api_token: &str, catalog_id: &str) -> Self {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(format!("Api-Key {}", api_token).as_str()).unwrap(),
        );

        Self {
            catalog_id: catalog_id.to_string(),
            client: Client::builder().default_headers(headers).build().unwrap(),
        }
    }

    pub async fn completion_async(
        &self,
        system_msg: String,
        user_msg: String,
    ) -> Result<String, String> {
        let response = fetch_with_retry(|| async {
            self.client
                .post(format!(
                    "{}/foundationModels/v1/completionAsync",
                    AI_FOUNDATION_MODELS_ENDPOINT
                ))
                .json(&json!({
                    "modelUri": ModelUri::YandexGPTPro(self.catalog_id.clone()).to_string(),
                    "completionOptions": {
                        "stream": false,
                        "temperature": 0,
                        "maxTokens": 10000
                    },
                    "messages": [
                        {
                            "role": "system",
                            "text": system_msg
                        },
                        {
                            "role": "user",
                            "text": user_msg
                        }
                    ]
                }))
                .send()
                .await
        })
            .await?;

        let body = response.text().await.map_err(|_| String::from("Invalid text response"))?;

        let Operation { id, .. } = serde_json::from_str::<Operation>(body.as_str())
            .map_err(|err| err.to_string())?;

        Ok(id)
    }

    pub async fn operation(&self, operation_id: String) -> Result<Operation, String> {
        let response = fetch_with_retry(|| async {
            self.client
                .get(format!(
                    "{}/operations/{}",
                    OPERATION_ENDPOINT, operation_id
                ))
                .send()
                .await
        })
            .await?;

        let body = response.text().await.map_err(|_| String::from("Invalid text response"))?;

        let operation = serde_json::from_str::<Operation>(body.as_str())
            .map_err(|err| {
                log::error!("Operation parse error: {:?}", err);

                err.to_string()
            })?;

        Ok(operation)
    }
}
