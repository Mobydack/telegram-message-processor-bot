use std::{future::Future, ops::Add};
use tokio::time::{timeout, Duration};
use yandex_cloud_grpc_client::tonic_exports::{Code, Status};

#[derive(Debug, Clone)]
pub struct RetryConfiguration {
    pub max_retries: u8,
    pub base_duration: Duration,
    attempts: u8,
    next_duration: Duration,
}

impl RetryConfiguration {
    pub fn new(max_retries: u8, base_duration: Duration) -> Self {
        RetryConfiguration {
            max_retries,
            base_duration,
            attempts: 0,
            next_duration: base_duration,
        }
    }

    pub fn update(&mut self) -> Result<(), Status> {
        if self.attempts >= self.max_retries {
            return Err(Status::aborted("Reached maximum number of retries."));
        }

        self.attempts += 1;

        self.next_duration = if self.next_duration.is_zero() {
            self.next_duration.add(Duration::from_secs(1))
        } else {
            self.next_duration.mul_f64(1.5)
        };

        Ok(())
    }
}

fn should_retry(status: &Status) -> bool {
    matches!(status.code(), Code::Unavailable | Code::DeadlineExceeded | Code::Internal | Code::Unknown)
}

pub async fn retry_grpc_request<T, F, Fut>(
    mut func: F,
    retry_conf: &mut RetryConfiguration,
) -> Result<T, Status> where
    Fut: Future<Output = Result<T, Status>>,
    F: FnMut() -> Fut,
{
    loop {
        match timeout(retry_conf.next_duration, func()).await {
            Ok(Ok(result)) => {
                return Ok(result);
            },
            Ok(Err(error)) => {
                match should_retry(&error) {
                    true => {
                        if let Err(err) = retry_conf.update() {
                            return Err(err);
                        }
                    }
                    false => {
                        return Err(error);
                    }
                }
            },
            Err(_) => {
                if let Err(err) = retry_conf.update() {
                    return Err(err);
                }
            },
        }
    }
}
