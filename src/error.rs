
use std::{fmt::{self, Display, Debug}, ops::{Deref, DerefMut}};

/// A generic error type.
pub struct Error(anyhow::Error);

impl Error {
    /// Use this when you need to implement middleware, where the error type must be `surf::Error`.
    pub fn new<E>(error: E) -> Self
        where E: std::error::Error + Send + Sync + 'static
    {
        Self(anyhow::Error::new(error))
    }
    /// Use this to create string errors.
    pub(crate) fn msg<M>(message: M) -> Self
        where M: Display + Debug + Send + Sync + 'static
    {
        Self(anyhow::Error::msg(message))
    }

    /// Use this to add context to errors
    #[allow(dead_code)]
    pub(crate) fn context<C>(self, context: C) -> Self
        where C: Display + Send + Sync + 'static
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

impl Deref for Error {
    type Target = dyn std::error::Error + Send + Sync + 'static;
    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.0)
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        DerefMut::deref_mut(&mut self.0)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
