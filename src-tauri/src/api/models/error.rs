use std::sync::PoisonError;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("vigem_client failure :: {0}")]
    VigemClient(#[from] vigem_client::Error),
    #[error("poison_error :: {0}")]
    StateAccessError(String),
    #[error("failed to get value from option :: {0}")]
    OptionRetrieveError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::StateAccessError(String::from(value.to_string()))
    }
}
