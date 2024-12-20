use crate::Json;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use worker::Error as WorkerError;

pub enum Error {
    JsonRejection(JsonRejection),
    WorkerError(WorkerError),
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            code: u16,
            message: String,
        }

        let (code, message) = match self {
            Error::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            Error::WorkerError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        Json(ErrorResponse {
            code: code.as_u16(),
            message,
        })
        .with_status_code(code)
        .into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}
