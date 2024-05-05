use backoff::{future::retry, Error, ExponentialBackoff};
use reqwest::Response;
use std::future::Future;

pub async fn fetch_with_retry<Op, Fut>(operation: Op) -> Result<Response, String>
where
    Op: Fn() -> Fut,
    Fut: Future<Output = reqwest::Result<Response>>,
{
    retry(ExponentialBackoff::default(), || async {
        let response = operation()
            .await
            .map_err(|err| Error::Permanent(err.to_string()))?;

        let status_code = response.status();

        if status_code.is_client_error() {
            let canonical_reason = status_code
                .canonical_reason()
                .map(|value| value.to_string())
                .unwrap_or(format!(
                    "Client error with status code: [{}]",
                    status_code.as_str()
                ));

            match status_code {
                reqwest::StatusCode::TOO_MANY_REQUESTS => Err(Error::Transient {
                    err: canonical_reason,
                    retry_after: None,
                }),
                _ => Err(Error::Permanent(canonical_reason)),
            }
        } else if status_code.is_server_error() {
            Err(Error::Permanent(
                status_code
                    .canonical_reason()
                    .map(|value| value.to_string())
                    .unwrap_or(format!(
                        "Internal server error with status code: [{}]",
                        status_code.as_str()
                    )),
            ))
        } else {
            Ok(response)
        }
    })
    .await
}
