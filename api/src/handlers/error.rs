use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    BadRequest(&'static str),
    NotFound(&'static str),
    ServiceUnavailable(&'static str),
    BadGateway {
        service: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub trait ApiServiceError: std::error::Error + Send + Sync + 'static {
    const SERVICE: &'static str;
}

impl ApiError {
    pub fn bad_request(message: &'static str) -> Self {
        Self::BadRequest(message)
    }

    pub fn not_found(message: &'static str) -> Self {
        Self::NotFound(message)
    }

    pub fn service_unavailable(message: &'static str) -> Self {
        Self::ServiceUnavailable(message)
    }

    pub fn bad_gateway(
        service: &'static str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::BadGateway {
            service,
            source: Box::new(source),
        }
    }
}

impl<T> From<T> for ApiError
where
    T: ApiServiceError,
{
    fn from(error: T) -> Self {
        Self::bad_gateway(T::SERVICE, error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
            Self::ServiceUnavailable(message) => {
                (StatusCode::SERVICE_UNAVAILABLE, message).into_response()
            }
            Self::BadGateway { service, source } => {
                eprintln!("{service} service error: {source}");
                (
                    StatusCode::BAD_GATEWAY,
                    format!("failed to fetch {service} data"),
                )
                    .into_response()
            }
        }
    }
}
