use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use domain::app_errors::{AppError, OutputPortError};
use serde::Serialize;

pub enum ApiError {
    App(AppError),
    InputPort(InputPortError),
}

pub enum InputPortError {
    Parse(anyhow::Error),
    Contract(anyhow::Error),
}

#[derive(Serialize)]
pub enum Error {
    Public { error: String, message: String },
    Private { error: String },
}

impl From<AppError> for ApiError {
    fn from(error: AppError) -> Self {
        Self::App(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, Error) = match self {
            ApiError::App(error) => match error {
                AppError::InputContract(x) => (
                    StatusCode::BAD_REQUEST,
                    Error::Public {
                        error: "inputContractError".to_string(),
                        message: x.to_string(),
                    },
                ),
                AppError::Authorization(x) => (
                    StatusCode::FORBIDDEN,
                    Error::Public {
                        error: "authorization".to_string(),
                        message: x,
                    },
                ),
                AppError::Checkout(x) => (
                    StatusCode::BAD_REQUEST,
                    Error::Public {
                        error: "checkoutError".to_string(),
                        message: x,
                    },
                ),
                AppError::DomainInvariant(x) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::Private {
                        error: "internalServerError".to_string(),
                    },
                ),
                AppError::OutputPort(x) => match x {
                    OutputPortError::Connection(x) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::Private {
                            error: "internalServerError".to_string(),
                        },
                    ),
                    OutputPortError::NotFound(x) => (
                        StatusCode::BAD_REQUEST,
                        Error::Public {
                            error: "notFoundError".to_string(),
                            message: x.to_string(),
                        },
                    ),
                    OutputPortError::Contract(x) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::Private {
                            error: "contractError".to_string(),
                        },
                    ),
                    OutputPortError::Access(x) => (
                        StatusCode::FORBIDDEN,
                        Error::Public {
                            error: "authorization".to_string(),
                            message: x.to_string(),
                        },
                    ),
                    OutputPortError::Other(x) => (
                        StatusCode::BAD_REQUEST,
                        Error::Public {
                            error: "inputContractError".to_string(),
                            message: x.to_string(),
                        },
                    ),
                },
            },
            ApiError::InputPort(error) => match error {
                InputPortError::Parse(x) => (
                    StatusCode::BAD_REQUEST,
                    Error::Public {
                        error: "inputContractError".to_string(),
                        message: x.to_string(),
                    },
                ),
                InputPortError::Contract(x) => (
                    StatusCode::BAD_REQUEST,
                    Error::Public {
                        error: "inputContractError".to_string(),
                        message: x.to_string(),
                    },
                ),
            },
        };

        (status, Json(message)).into_response()
    }
}
