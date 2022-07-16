use std::io;

use serde::Serialize;
use tauri::InvokeError;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    IOError(String),
    DbError(String),
    HysteriaError(String),
    GitError(String),
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::GitError(err.message().to_owned())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::DbError(err.to_string())
    }
}

impl From<InvokeError> for Error {
    fn from(err: InvokeError) -> Self {
        Error::HysteriaError(format!("{:?}", err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::HysteriaError(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::HysteriaError(err.to_owned())
    }
}

impl Into<String> for Error {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}
