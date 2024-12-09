use std::error::Error;
use std::ffi;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DropRootError {
    IoError(io::Error),
    NulError(ffi::NulError),
    InvalidData,
}

impl DropRootError {
    pub(crate) fn last_os_error() -> Self {
        let error = io::Error::last_os_error();
        let os_error = error.raw_os_error();

        if os_error.is_none() || os_error == Some(0) {
            Self::InvalidData
        } else {
            Self::IoError(error)
        }
    }
}

impl From<ffi::NulError> for DropRootError {
    fn from(error: ffi::NulError) -> Self {
        Self::NulError(error)
    }
}

impl fmt::Display for DropRootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(error) => error.fmt(f),
            Self::NulError(_) => write!(f, "Cannot create CString from String"),
            Self::InvalidData => write!(f, "Bad user or group.")
        }
    }
}

impl Error for DropRootError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError(error) => Some(error),
            Self::NulError(error) => Some(error),

            Self::InvalidData => None,
        }
    }
}
