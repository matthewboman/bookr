use actix_web::{ResponseError, http::StatusCode};
use std::convert::From;
use tokio::task::JoinError;

use crate::auth::AuthError;
use crate::utils::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum AdminError {
    #[error("{0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid token")]
    InvalidToken,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("{0}")]
    ValidationError(String),
}

impl ResponseError for AdminError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl std::fmt::Debug for AdminError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum ContentError {
    #[error("User tried editting unauthorized content")]
    AuthorizationError,

    #[error("{0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Could not connect to Redis")]
    RedisError(#[from] redis::RedisError),

    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),

    #[error("{0}")]
    ValidationError(String),
}

impl ResponseError for ContentError {
    fn status_code(&self) -> StatusCode {
        match self {
            ContentError::AuthorizationError => StatusCode::UNAUTHORIZED,
            ContentError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ContentError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ContentError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ContentError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl std::fmt::Debug for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum GeocodingError {
    #[error("An error occurred while contacting the API: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("No results found for the address: {0}")]
    NoResultsFound(String),

    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GeocodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for GeocodingError {
    fn status_code(&self) -> StatusCode {
        match self {
            GeocodingError::ApiError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GeocodingError::NoResultsFound(_) => StatusCode::BAD_REQUEST,
            GeocodingError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[from] AuthError),

    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum RegisterError {
    #[error("Authentication failed")]
    AuthError(#[from] AuthError),

    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for RegisterError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegisterError::AuthError(_) => StatusCode::UNAUTHORIZED,
            RegisterError::ValidationError(_) => StatusCode::BAD_REQUEST,
            RegisterError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<JoinError> for RegisterError {
    fn from(_: JoinError) -> Self {
        RegisterError::ValidationError("Join error occurred".to_string())
    }
}

#[derive(thiserror::Error)]
pub enum TokenError {
    #[error("{0}")] 
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid token")]
    InvalidToken,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("A user with this email could not be found")]
    UnknownEmail,

    #[error("{0}")]
    ValidationError(String),
}

impl ResponseError for TokenError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnknownEmail => StatusCode::UNAUTHORIZED,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
           
        }
    }
}

impl std::fmt::Debug for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<JoinError> for TokenError {
    fn from(_: JoinError) -> Self {
        TokenError::ValidationError("Error hashing password".to_string())
    }
}