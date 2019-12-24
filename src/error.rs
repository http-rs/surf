//! The error types for `surf`.
//!
//! This module contains the type `Error` which represents any error that surf may return. It also
//! contains a `Result` type alias which is like `std::result::Result` but defaults to using
//! `surf::Error`. `Error` and `Result` are re-exported at the crate root, so you shouldn't
//! usually have to use anything in this module.
use std::fmt::{self, Debug, Display};

/// A version of `std::result::Result` that defaults the error type to `surf::Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A generic error type.
pub struct Error(pub(crate) anyhow::Error);

impl Error {
    /// Use this when you need to implement middleware, where the error type must be `surf::Error`.
    pub(crate) fn new<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self(anyhow::Error::new(error))
    }

    /// Use this to create string errors.
    pub(crate) fn msg<M>(message: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self(anyhow::Error::msg(message))
    }

    /// Use this to add context to errors
    #[allow(dead_code)]
    pub(crate) fn context<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        Self(anyhow::Error::context(self.0, context))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(error: log::SetLoggerError) -> Self {
        Self::new(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new(error)
    }
}
