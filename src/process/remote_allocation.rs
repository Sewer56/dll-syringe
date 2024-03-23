use crate::errors::structs::OsError;

use super::{BorrowedProcess, OwnedProcess, Process};
use reloaded_memory_buffers::structs::PrivateAllocation;
use std::os::windows::io::AsRawHandle;
use winapi::shared::minwindef::{FALSE, LPVOID};
use winapi::um::memoryapi::WriteProcessMemory;

/// Represents a slice of memory in a remote process.
pub struct RemoteAllocation {
    allocation: PrivateAllocation,
    offset: usize,
    process: OwnedProcess,
}

impl RemoteAllocation {
    /// Creates a new `RemoteAllocation` for a given `PrivateAllocation`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `PrivateAllocation` indeed belongs to a remote process
    /// and that the process is accessible and allows memory operations.
    ///
    /// # Arguments
    ///
    /// * `allocation` - A `PrivateAllocation` instance representing allocated memory in a remote process.
    /// * `process` - The target process where the memory is allocated.
    pub fn new(allocation: PrivateAllocation, process: OwnedProcess) -> Self {
        Self {
            allocation,
            offset: 0,
            process,
        }
    }

    /// Flushes the CPU instruction cache for the memory region represented by this allocation.
    ///
    /// This operation ensures that if the memory modified by this `RemoteAllocation` instance
    /// was used to store executable code, the changes are recognized by the CPU. Flushing the
    /// instruction cache is crucial on architectures with a non-unified memory cache (where
    /// the data cache is separate from the instruction cache), such as ARM, to ensure that
    /// any modifications to executable code are visible to the processor when executed.
    ///
    /// On x86 and x86_64 architectures, the cache is unified, and modifications to memory
    /// are automatically visible to the instruction stream; therefore, calling this function
    /// is not strictly necessary but is safe to do.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the instruction cache was successfully flushed, or a
    /// `RemoteAllocationError` containing the error code if the operation failed.
    pub fn flush_instruction_cache(&self) -> Result<(), RemoteAllocationError> {
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            use winapi::um::processthreadsapi::FlushInstructionCache;
            let result = unsafe {
                FlushInstructionCache(
                    self.process.as_raw_handle(),
                    self.allocation.base_address.as_ptr().cast(),
                    self.allocation.size,
                )
            };
            if result == 0 {
                return Err(RemoteAllocationError::new());
            }
        }

        Ok(())
    }

    /// Appends data to the remote memory allocation and advances the internal pointer.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to write into the remote process's memory.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the data was successfully written, or a `RemoteAllocationError` if
    /// an error occurred during the write operation.
    pub fn append(&mut self, data: &[u8]) -> Result<usize, RemoteAllocationError> {
        let write_addr = unsafe { self.allocation.base_address.as_ptr().add(self.offset) as usize };
        self.write_at(self.offset, data)?;
        self.offset += data.len();
        Ok(write_addr)
    }

    /// Writes data to the remote memory allocation at the current pointer without advancing it.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to write into the remote process's memory at the current offset.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the data was successfully written, or a `RemoteAllocationError` if
    /// an error occurred during the write operation.
    pub fn write_in_place(&mut self, data: &[u8]) -> Result<usize, RemoteAllocationError> {
        let write_addr = unsafe { self.allocation.base_address.as_ptr().add(self.offset) as usize };
        self.write_at(self.offset, data);
        Ok(write_addr)
    }

    /// Borrows the process stored in this `RemoteAllocation`.
    pub fn process(&self) -> BorrowedProcess<'_> {
        self.process.borrowed()
    }

    /// Writes data to the allocation at a specific offset.
    fn write_at(&self, offset: usize, data: &[u8]) -> Result<(), RemoteAllocationError> {
        let mut bytes_written: usize = 0;
        let write_result = unsafe {
            WriteProcessMemory(
                self.process.as_raw_handle(),
                self.allocation.base_address.as_ptr().add(offset) as LPVOID,
                data.as_ptr() as *const _,
                data.len(),
                &mut bytes_written as *mut usize as *mut _,
            )
        };

        if write_result == FALSE {
            return Err(RemoteAllocationError::new());
        }

        Ok(())
    }
}

/// Represents an error that occurred during a remote allocation operation.
pub struct RemoteAllocationError {
    /// The raw error code from 'GetLastError' call.
    pub os_error: OsError,
}

impl RemoteAllocationError {
    /// Returns the raw error code from 'GetLastError' call.
    #[must_use]
    pub fn new() -> Self {
        return RemoteAllocationError {
            os_error: OsError::new(),
        };
    }
}
