use reloaded_memory_buffers::structs::errors::BufferAllocationError;

use crate::{errors::structs::OsError, process::RemoteAllocationError};

/// Errors that can occur when creating a `Syringe` instance.
pub enum CreateSyringeError {
    /// Failed to allocate buffer in remote process.
    BufferAllocationError(BufferAllocationError),

    /// Operating system related error.
    OsError(OsError),

    /// Error in allocating memory
    RemoteAllocationError(RemoteAllocationError),
}

impl From<BufferAllocationError> for CreateSyringeError {
    fn from(error: BufferAllocationError) -> Self {
        CreateSyringeError::BufferAllocationError(error)
    }
}

impl From<OsError> for CreateSyringeError {
    fn from(error: OsError) -> Self {
        CreateSyringeError::OsError(error)
    }
}

impl From<RemoteAllocationError> for CreateSyringeError {
    fn from(error: RemoteAllocationError) -> Self {
        CreateSyringeError::RemoteAllocationError(error)
    }
}
