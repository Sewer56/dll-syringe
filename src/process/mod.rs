mod process;
pub use process::*;

mod owned;
pub use owned::*;

mod borrowed;
pub use borrowed::*;

mod module;
pub use module::*;

#[cfg_attr(not(feature = "process-memory"), allow(dead_code))]
#[cfg(not(feature = "process-memory"))]
/// Module containing utilities for dealing with memory of another process.
pub(crate) mod memory;

mod process_memory_slice;
#[allow(unused_imports)]
pub use process_memory_slice::*;

mod remote_allocation;
pub use remote_allocation::*;
