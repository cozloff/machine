use std::{error::Error, fmt};

#[derive(Debug)]
pub enum InflationServiceError {
    Request(reqwest::Error),
    Parse(serde_json::Error),
    UnexpectedStatus(reqwest::StatusCode),
}

impl From<reqwest::Error> for InflationServiceError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<serde_json::Error> for InflationServiceError {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

impl fmt::Display for InflationServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(error) => write!(formatter, "request failed: {error}"),
            Self::Parse(error) => write!(formatter, "failed to parse response: {error}"),
            Self::UnexpectedStatus(status) => write!(formatter, "unexpected status: {status}"),
        }
    }
}

impl Error for InflationServiceError {}
