use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    NotFound(&'static str),
    BadGateway {
        service: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub trait ApiServiceError: std::error::Error + Send + Sync + 'static {
    const SERVICE: &'static str;
}

impl ApiError {
    pub fn not_found(message: &'static str) -> Self {
        Self::NotFound(message)
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
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
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
