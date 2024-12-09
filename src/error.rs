use std::error::Error;
use std::ffi;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DropRootError {
    IoError(io::Error),
    NulError(ffi::NulError),
}

impl DropRootError {
    pub(crate) fn last_os_error() -> Self {
        Self::IoError(io::Error::last_os_error())
    }

    pub(crate) fn invalid_string(error: ffi::NulError) -> Self {
        Self::NulError(error)
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
        }
    }
}

impl Error for DropRootError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError(error) => Some(error),
            Self::NulError(error) => Some(error),
        }
    }
}
