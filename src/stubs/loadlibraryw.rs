use core::{mem, ptr::write_unaligned};

pub(crate) type LoadLibraryWFn = unsafe extern "system" fn(LPCWSTR) -> HMODULE;
use crate::{
    error::InjectError,
    process::{
        memory::{RemoteAllocation, RemoteBox, RemoteBoxAllocator},
        BorrowedProcess, ModuleHandle, Process,
    },
    InjectHelpData,
};
use crate::{GetLastErrorFn, Syringe};
use winapi::{shared::minwindef::HMODULE, um::winnt::LPCWSTR};

const X64_CODE: &[u8; 48] = include_bytes!("./loadlibraryw.x64.bin");
const X86_CODE: &[u8; 31] = include_bytes!("./loadlibraryw.x86.bin");

#[derive(Debug)]
pub(crate) struct LoadLibraryWStub {
    code: RemoteAllocation,
    result: RemoteBox<ModuleHandle>,
}

impl LoadLibraryWStub {
    pub(crate) fn build(
        inject_data: &InjectHelpData,
        remote_allocator: &RemoteBoxAllocator,
    ) -> Result<Self, InjectError> {
        let result = remote_allocator.alloc_uninit::<ModuleHandle>()?;

        // TODO: If current process is x86, no point including x64 code path.
        if remote_allocator.process().is_x86()? {
            let code = Self::build_code_x86(
                inject_data.get_load_library_fn_ptr(),
                result.as_raw_ptr().cast(),
                inject_data.get_get_last_error(),
            );
            let code = remote_allocator.alloc_and_copy_buf(code.as_slice())?;
            Ok(Self { code, result })
        } else {
            let code = Self::build_code_x64(
                inject_data.get_load_library_fn_ptr(),
                result.as_raw_ptr().cast(),
                inject_data.get_get_last_error(),
            );
            let code = remote_allocator.alloc_and_copy_buf(code.as_slice())?;
            Ok(Self { code, result })
        }
    }

    pub(crate) fn call(
        &self,
        remote_wide_module_path: *mut u16,
    ) -> Result<ModuleHandle, InjectError> {
        // creating a thread that will call LoadLibraryW with a pointer to payload_path as argument
        let exit_code = self.code.process().run_remote_thread(
            unsafe { mem::transmute(self.code.as_raw_ptr()) },
            remote_wide_module_path,
        )?;

        Syringe::remote_exit_code_to_error_or_exception(exit_code)?;

        let injected_module_handle = self.result.read()?;
        assert!(!injected_module_handle.is_null());

        Ok(injected_module_handle)
    }

    #[allow(dead_code)]
    fn process(&self) -> BorrowedProcess<'_> {
        self.code.process()
    }

    #[allow(clippy::fn_to_numeric_cast, clippy::fn_to_numeric_cast_with_truncation)]
    fn build_code_x86(
        load_library_w: LoadLibraryWFn,
        return_buffer: *mut HMODULE,
        get_last_error: GetLastErrorFn,
    ) -> [u8; 31] {
        debug_assert!(!return_buffer.is_null());

        // Overflow check
        debug_assert_eq!(load_library_w as u32 as usize, load_library_w as usize);
        debug_assert_eq!(return_buffer as u32 as usize, return_buffer as usize);
        debug_assert_eq!(get_last_error as u32 as usize, get_last_error as usize);

        let mut code: [u8; 31] = [0; 31]; // zero fill eliminated by compiler
        code.copy_from_slice(X86_CODE);

        unsafe {
            write_unaligned(code.as_mut_ptr().add(6) as *mut u32, load_library_w as u32);
            write_unaligned(code.as_mut_ptr().add(13) as *mut u32, return_buffer as u32);
            write_unaligned(code.as_mut_ptr().add(22) as *mut u32, get_last_error as u32);
        }

        code
    }

    #[allow(clippy::fn_to_numeric_cast, clippy::fn_to_numeric_cast_with_truncation)]
    fn build_code_x64(
        load_library_w: LoadLibraryWFn,
        return_buffer: *mut HMODULE,
        get_last_error: GetLastErrorFn,
    ) -> [u8; 48] {
        debug_assert!(!return_buffer.is_null());
        let mut code: [u8; 48] = [0; 48]; // zero fill eliminated by compiler
        code.copy_from_slice(X64_CODE);

        unsafe {
            write_unaligned(code.as_mut_ptr().add(6) as *mut u64, load_library_w as u64);
            write_unaligned(code.as_mut_ptr().add(18) as *mut u64, return_buffer as u64);
            write_unaligned(code.as_mut_ptr().add(33) as *mut u64, get_last_error as u64);
        }

        code
    }
}
