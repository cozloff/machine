use std::{error::Error, fmt};

use crate::repositories::population::PopulationRepositoryError;

#[derive(Debug)]
pub enum PopulationServiceError {
    Request(reqwest::Error),
    Parse(serde_json::Error),
    Repository(PopulationRepositoryError),
    UnexpectedStatus(reqwest::StatusCode),
}

impl From<reqwest::Error> for PopulationServiceError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<serde_json::Error> for PopulationServiceError {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

impl From<PopulationRepositoryError> for PopulationServiceError {
    fn from(error: PopulationRepositoryError) -> Self {
        Self::Repository(error)
    }
}

impl fmt::Display for PopulationServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(error) => write!(formatter, "request failed: {error}"),
            Self::Parse(error) => write!(formatter, "failed to parse response: {error}"),
            Self::Repository(error) => write!(formatter, "repository error: {error}"),
            Self::UnexpectedStatus(status) => write!(formatter, "unexpected status: {status}"),
        }
    }
}

impl Error for PopulationServiceError {}
