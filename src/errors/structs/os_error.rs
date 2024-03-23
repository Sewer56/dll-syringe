use std::io;

/// Represents an error that occurred during a remote allocation operation.
pub struct OsError {
    /// The raw error code from 'GetLastError' call.
    pub last_os_error: u32,
}

impl OsError {
    /// Returns the raw error code from 'GetLastError' call.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            return OsError {
                last_os_error: io::Error::last_os_error().raw_os_error().unwrap_unchecked() as u32,
            };
        }
    }
}

impl From<io::Error> for OsError {
    fn from(error: io::Error) -> Self {
        OsError {
            last_os_error: error.raw_os_error().unwrap_or(0) as u32,
        }
    }
}
