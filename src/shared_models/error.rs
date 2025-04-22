use axum::{http::StatusCode, response::IntoResponse};
use std::{num, sync::PoisonError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("vigem_client failure :: {0}")]
    VigemClient(#[from] vigem_client::Error),
    #[error("poison_error :: {0}")]
    StateAccess(String),
    #[error("failed to get value from option :: {0}")]
    OptionRetrieve(String),
    #[error("failed on int conversion :: {0}")]
    TryFromInt(#[from] num::TryFromIntError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::StateAccess(value.to_string())
    }
}
