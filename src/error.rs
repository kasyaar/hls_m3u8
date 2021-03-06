use trackable::error::{ErrorKind as TrackableErrorKind, TrackableError};

/// This crate specific `Error` type.
#[derive(Debug, Clone, TrackableError)]
pub struct Error(TrackableError<ErrorKind>);

/// Possible error kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ErrorKind {
    InvalidInput,
}
impl TrackableErrorKind for ErrorKind {}
