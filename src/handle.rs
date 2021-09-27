use std::cmp;
use std::io;
use std::ptr;

use crate::bindings::{
    Windows::Win32::Foundation::*, Windows::Win32::Storage::FileSystem::*,
    Windows::Win32::System::Diagnostics::Debug::*, Windows::Win32::System::SystemServices::*,
};

#[derive(Debug)]
pub struct Handle(HANDLE);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Handle {
    pub fn new(handle: HANDLE) -> Handle {
        Handle(handle)
    }

    pub fn raw(&self) -> HANDLE {
        self.0
    }

    pub fn into_raw(self) -> HANDLE {
        use std::mem;

        let ret = self.0;
        mem::forget(self);
        ret
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes = 0;
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
        unsafe {
            WriteFile(
                self.0,
                buf.as_ptr() as *const _,
                len,
                &mut bytes,
                0 as *mut _,
            )
            .ok()?;
        }
        Ok(bytes as usize)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes = 0;
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
        unsafe {
            ReadFile(
                self.0,
                buf.as_mut_ptr() as *mut _,
                len,
                &mut bytes,
                0 as *mut _,
            )
            .ok()?;
        }
        Ok(bytes as usize)
    }

    pub unsafe fn read_overlapped(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        self.read_overlapped_helper(buf, overlapped, false)
    }

    pub unsafe fn read_overlapped_wait(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<usize> {
        match self.read_overlapped_helper(buf, overlapped, true) {
            Ok(Some(bytes)) => Ok(bytes),
            Ok(None) => panic!("logic error"),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn read_overlapped_helper(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
        wait: bool,
    ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
        let res = ReadFile(
            self.0,
            buf.as_mut_ptr() as *mut _,
            len,
            ptr::null_mut(),
            overlapped,
        )
        .ok();
        match res {
            Ok(_) => (),
            Err(ref e) if e.win32_error() == Some(ERROR_IO_PENDING.0) => (),
            Err(e) => return Err(e.into()),
        }

        let mut bytes = 0;
        let res = GetOverlappedResult(self.0, overlapped, &mut bytes, wait).ok();
        match res {
            Ok(_) => Ok(Some(bytes as usize)),
            Err(ref e) if e.win32_error() == Some(ERROR_IO_INCOMPLETE.0) && !wait => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub unsafe fn write_overlapped(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        self.write_overlapped_helper(buf, overlapped, false)
    }

    pub unsafe fn write_overlapped_wait(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<usize> {
        match self.write_overlapped_helper(buf, overlapped, true) {
            Ok(Some(bytes)) => Ok(bytes),
            Ok(None) => panic!("logic error"),
            Err(e) => Err(e),
        }
    }

    unsafe fn write_overlapped_helper(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
        wait: bool,
    ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
        let res = WriteFile(
            self.0,
            buf.as_ptr() as *const _,
            len,
            ptr::null_mut(),
            overlapped,
        )
        .ok();
        match res {
            Ok(_) => (),
            Err(ref e) if e.win32_error() == Some(ERROR_IO_PENDING.0) => (),
            Err(e) => return Err(e.into()),
        }

        let mut bytes = 0;
        let res = GetOverlappedResult(self.0, overlapped, &mut bytes, wait).ok();
        match res {
            Ok(_) => Ok(Some(bytes as usize)),
            Err(ref e) if e.win32_error() == Some(ERROR_IO_INCOMPLETE.0) && !wait => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
