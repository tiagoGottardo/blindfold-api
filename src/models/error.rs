use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct ErrorDefault {
    code: StatusCode,
    message: String,
}

impl ErrorDefault {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

impl IntoResponse for ErrorDefault {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

impl From<std::io::Error> for ErrorDefault {
    fn from(err: std::io::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}
