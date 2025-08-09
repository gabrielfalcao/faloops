use std::fmt::Display;

use cpal::{
    BackendSpecificError, BuildStreamError, DefaultStreamConfigError, DeviceNameError,
    DevicesError, PauseStreamError, PlayStreamError, StreamError,
    SupportedStreamConfigsError,
};

#[derive(Debug, Clone)]
pub enum Error {
    BackendSpecificError(String),
    BuildStreamError(String),
    DefaultStreamConfigError(String),
    DeviceNameError(String),
    DevicesError(String),
    PauseStreamError(String),
    PlayStreamError(String),
    StreamError(String),
    RandomError(String),
    RuntimeError(String),
    SupportedStreamConfigsError(String),
}
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::BackendSpecificError(e) => e.to_string(),
                Error::BuildStreamError(e) => e.to_string(),
                Error::DefaultStreamConfigError(e) => e.to_string(),
                Error::DeviceNameError(e) => e.to_string(),
                Error::DevicesError(e) => e.to_string(),
                Error::PauseStreamError(e) => e.to_string(),
                Error::PlayStreamError(e) => e.to_string(),
                Error::StreamError(e) => e.to_string(),
                Error::RandomError(e) => e.to_string(),
                Error::RuntimeError(e) => e.to_string(),
                Error::SupportedStreamConfigsError(e) => e.to_string(),
            }
        )
    }
}

impl From<BackendSpecificError> for Error {
    fn from(e: BackendSpecificError) -> Self {
        Error::BackendSpecificError(e.to_string())
    }
}

impl From<BuildStreamError> for Error {
    fn from(e: BuildStreamError) -> Self {
        Error::BuildStreamError(e.to_string())
    }
}

impl From<DefaultStreamConfigError> for Error {
    fn from(e: DefaultStreamConfigError) -> Self {
        Error::DefaultStreamConfigError(e.to_string())
    }
}

impl From<DeviceNameError> for Error {
    fn from(e: DeviceNameError) -> Self {
        Error::DeviceNameError(e.to_string())
    }
}

impl From<DevicesError> for Error {
    fn from(e: DevicesError) -> Self {
        Error::DevicesError(e.to_string())
    }
}

impl From<PauseStreamError> for Error {
    fn from(e: PauseStreamError) -> Self {
        Error::PauseStreamError(e.to_string())
    }
}

impl From<PlayStreamError> for Error {
    fn from(e: PlayStreamError) -> Self {
        Error::PlayStreamError(e.to_string())
    }
}

impl From<StreamError> for Error {
    fn from(e: StreamError) -> Self {
        Error::StreamError(e.to_string())
    }
}

impl From<SupportedStreamConfigsError> for Error {
    fn from(e: SupportedStreamConfigsError) -> Self {
        Error::SupportedStreamConfigsError(e.to_string())
    }
}
impl From<rand::distr::uniform::Error> for Error {
    fn from(e: rand::distr::uniform::Error) -> Self {
        Error::RandomError(e.to_string())
    }
}

impl From<ctrlc::Error> for Error {
    fn from(e: ctrlc::Error) -> Self {
        Error::RuntimeError(e.to_string())
    }
}
