use std::result::Result as StdResult;

use thiserror::Error as ThisError;

#[allow(missing_docs)]
pub type Result<T> = StdResult<T, Error>;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
#[error(transparent)]
pub struct Error(#[from] InternalError);

macro_rules! impl_from {
    ($bound:ty) => {
        impl From<$bound> for Error {
            fn from(err: $bound) -> Error {
                InternalError::from(err).into()
            }
        }
    };
}

impl_from!(log::kv::Error);
impl_from!(serde_urlencoded::ser::Error);
impl_from!(serde_urlencoded::de::Error);
impl_from!(std::io::Error);
impl_from!(serde_json::Error);

#[derive(ThisError, Debug)]
pub(crate) enum InternalError {
    #[error(transparent)]
    LogError(#[from] log::kv::Error),
    #[error(transparent)]
    UrlEncodeError(#[from] serde_urlencoded::ser::Error),
    #[error(transparent)]
    UrlDecodeError(#[from] serde_urlencoded::de::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    HttpError(String),
}
