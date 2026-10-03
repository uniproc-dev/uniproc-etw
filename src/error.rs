use std::fmt;

/// A Win32 call that failed, and the code it failed with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    pub call: &'static str,
    pub code: u32,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn new(call: &'static str, code: u32) -> Self {
        Self { call, code }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = std::io::Error::from_raw_os_error(self.code as i32);
        write!(f, "{}: {message}", self.call)
    }
}

impl std::error::Error for Error {}
