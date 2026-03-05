use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema, Clone)]
pub enum ErrorStatus {
    #[serde(rename = "error")]
    Error,
}

#[derive(serde::Serialize, ToSchema, Clone)]
pub struct ErrorResponse {
    pub message: String,
    #[serde(default = "error")]
    pub status: ErrorStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip)]
    pub code: StatusCode,
}

impl ErrorResponse {
    pub fn unauthorized() -> Self {
        Self {
            message: "unauthorized".to_string(),
            status: ErrorStatus::Error,
            error_code: None,
            code: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn bad_request() -> Self {
        Self {
            message: "bad request".to_string(),
            status: ErrorStatus::Error,
            error_code: None,
            code: StatusCode::BAD_REQUEST,
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            message: "internal server error".to_string(),
            status: ErrorStatus::Error,
            error_code: None,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn forbidden() -> Self {
        Self {
            message: "forbidden".to_string(),
            status: ErrorStatus::Error,
            error_code: None,
            code: StatusCode::FORBIDDEN,
        }
    }

    pub fn not_found() -> Self {
        Self {
            message: "not found".to_string(),
            status: ErrorStatus::Error,
            error_code: None,
            code: StatusCode::NOT_FOUND,
        }
    }
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>, code: StatusCode) -> Self {
        Self {
            message: message.into(),
            status: ErrorStatus::Error,
            error_code: None,
            code,
        }
    }

    pub fn with_message(&mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self.clone()
    }

    pub fn with_error_code(&mut self, error_code: impl Into<String>) -> Self {
        self.error_code = Some(error_code.into());
        self.clone()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self)).into_response()
    }
}

#[derive(serde::Serialize, ToSchema, serde::Deserialize, Default, Clone)]
pub enum SuccessStatus {
    #[serde(rename = "success")]
    #[default]
    Success,
}

#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
pub struct DataResponse<T: ToSchema> {
    #[serde(bound(
        serialize = "T: serde::Serialize",
        deserialize = "T: serde::Deserialize<'de>"
    ))]
    pub data: T,
    #[serde(default)]
    pub status: SuccessStatus,
}

impl<'de, T: serde::Serialize + ToSchema + serde::Deserialize<'de>> From<T> for DataResponse<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            status: SuccessStatus::Success,
        }
    }
}

impl<'de, T: serde::Serialize + ToSchema + serde::Deserialize<'de>> IntoResponse
    for DataResponse<T>
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
