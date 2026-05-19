use std::{error::Error, fmt};

#[derive(Debug)]
pub enum MachineServiceError {
    Config(&'static str),
    InvalidInput(&'static str),
    Request(reqwest::Error),
    UnexpectedStatus {
        status: reqwest::StatusCode,
        body: String,
    },
}

impl From<reqwest::Error> for MachineServiceError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl fmt::Display for MachineServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(message) => write!(formatter, "configuration error: {message}"),
            Self::InvalidInput(message) => write!(formatter, "invalid input: {message}"),
            Self::Request(error) => write!(formatter, "request failed: {error}"),
            Self::UnexpectedStatus { status, body } => {
                write!(formatter, "unexpected status {status}: {body}")
            }
        }
    }
}

impl Error for MachineServiceError {}
