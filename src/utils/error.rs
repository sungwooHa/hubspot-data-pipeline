use thiserror::Error;

pub type Result<T> = std::result::Result<T, HubSpotError>;

#[derive(Error, Debug)]
pub enum HubSpotError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error: {status_code} - {message}")]
    ApiError { status_code: u16, message: String },

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Missing API key")]
    MissingApiKey,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid API key : {0}")]
    InvalidApiKey(String),

    #[error("Resource not found:")]
    ResourceNotFound,

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("File read error : {0}")]
    FileError(String),

    #[error("Url Parse Error : {0}")]
    UrlParseError(String),
}

impl HubSpotError {
    pub fn from_status_and_message(status: u16, message: String) -> Self {
        match status {
            401 => HubSpotError::InvalidApiKey(message),
            404 => HubSpotError::ResourceNotFound,
            429 => HubSpotError::RateLimitExceeded,
            _ => HubSpotError::ApiError {
                status_code: status,
                message,
            },
        }
    }
}
