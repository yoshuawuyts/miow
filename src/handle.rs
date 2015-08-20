use std::mem;
use std::io;

use winapi::*;
use kernel32::*;

pub struct Handle(HANDLE);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Handle {
    pub fn new(handle: HANDLE) -> Handle {
        Handle(handle)
    }

    pub fn raw(&self) -> HANDLE { self.0 }

    pub fn into_raw(self) -> HANDLE {
        let ret = self.0;
        mem::forget(self);
        ret
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes = 0;
        try!(::cvt(unsafe {
            WriteFile(self.0, buf.as_ptr() as *const _,
                      buf.len() as DWORD, &mut bytes, 0 as *mut _)
        }));
        Ok(bytes as usize)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes = 0;
        try!(::cvt(unsafe {
            ReadFile(self.0, buf.as_mut_ptr() as *mut _,
                     buf.len() as DWORD, &mut bytes, 0 as *mut _)
        }));
        Ok(bytes as usize)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
