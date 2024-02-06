use crate::services::models::{ConfigServiceError, StateServiceError};
use crate::{
    clients::ipfs::models::IpfsClientError,
    controllers::models::{AuthenticationError, RegisterError},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub struct AppResponse<T>(pub T);

pub enum AppError {
    ConfigServiceError(String),
    RegisterError(String),
    IpfsClientError(String),
    AuthenticationError(String),
    StateServiceError(String)
}

impl<T> IntoResponse for AppResponse<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = match self {
            Self::ConfigServiceError(err) => err,
            Self::RegisterError(err) => err,
            Self::IpfsClientError(err) => err,
            Self::AuthenticationError(err) => err,
            Self::StateServiceError(err) => err
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response();
    }
}

impl From<ConfigServiceError> for AppError {
    fn from(error: ConfigServiceError) -> Self {
        Self::ConfigServiceError(error.to_string())
    }
}

impl From<RegisterError> for AppError {
    fn from(error: RegisterError) -> Self {
        Self::RegisterError(error.to_string())
    }
}

impl From<IpfsClientError> for AppError {
    fn from(error: IpfsClientError) -> Self {
        Self::IpfsClientError(error.to_string())
    }
}

impl From<AuthenticationError> for AppError {
    fn from(error: AuthenticationError) -> Self {
        Self::AuthenticationError(error.to_string())
    }
}

impl From<StateServiceError> for AppError {
    fn from(error: StateServiceError) -> Self {
        Self::StateServiceError(error.to_string())
    }
}