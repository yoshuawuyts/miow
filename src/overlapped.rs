use std::{mem, io};

use winapi::*;

/// A wrapper around `OVERLAPPED` to provide "rustic" accessors and
/// initializers.
#[derive(Debug)]
pub struct Overlapped(OVERLAPPED);

unsafe impl Send for Overlapped {}
unsafe impl Sync for Overlapped {}

impl Overlapped {
    /// Creates a new zeroed out instance of an overlapped I/O tracking state.
    ///
    /// This is suitable for passing to methods which will then later get
    /// notified via an I/O Completion Port.
    pub fn zero() -> Overlapped {
        Overlapped(unsafe { mem::zeroed() })
    }

    /// Creates a new `Overlapped` function pointer from the underlying
    /// `OVERLAPPED`, wrapping in the "rusty" wrapper for working with
    /// accessors.
    ///
    /// # Unsafety
    ///
    /// This function doesn't validate `ptr` nor the lifetime of the returned
    /// pointer at all, it's recommended to use this method with extreme
    /// caution.
    pub unsafe fn from_raw<'a>(ptr: *mut OVERLAPPED) -> &'a mut Overlapped {
        &mut *(ptr as *mut Overlapped)
    }

    /// Gain access to the raw underlying data
    pub fn raw(&self) -> *mut OVERLAPPED {
        &self.0 as *const _ as *mut _
    }

    /// Sets the offset inside this overlapped structure.
    ///
    /// Note that for I/O operations in general this only has meaning for I/O
    /// handles that are on a seeking device that supports the concept of an
    /// offset.
    pub fn set_offset(&mut self, offset: u64) {
        self.0.Offset = offset as u32;
        self.0.OffsetHigh = (offset >> 32) as u32;
    }

    /// Reads the offset inside this overlapped structure.
    pub fn offset(&self) -> u64 {
        (self.0.Offset as u64) | ((self.0.OffsetHigh as u64) << 32)
    }

    /// Sets the `hEvent` field of this structure.
    ///
    /// The event specified can be null.
    pub fn set_event(&mut self, event: HANDLE) {
        self.0.hEvent = event;
    }

    /// Reads the `hEvent` field of this structure, may return null.
    pub fn event(&self) -> HANDLE {
        self.0.hEvent
    }

    /// Returns the status of the last I/O operation associated with
    /// this structure, or Ok(()) if none has yet been associated.
    ///
    /// Per the documentation at
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms684342(v=vs.85).aspx
    /// this operates via the Internal member. While this is
    /// documented as unstable, its use is necessary when
    /// GetQueuedCompletionStatusEx is used, and recommended in
    /// multiple instances including
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/aa364988(v=vs.85).aspx
    /// by msdn. We therefore judge it to be de-facto stable.
    pub fn status(&self) -> io::Result<()> {
        println!("{:X}", self.0.Internal);
        if self.0.Internal >= (1 << mem::size_of::<ntdef::NTSTATUS>() * 8) {
            // The documentation is unclear about what exactly might someday end up in this field.
            return Err(io::Error::new(io::ErrorKind::Other, "unknown error"));
        }

        let status = self.0.Internal as ntdef::NTSTATUS;
        if status == ntstatus::STATUS_SUCCESS {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(hresult_from_nt(status)))
        }
    }
}

fn hresult_from_nt(x: ntdef::NTSTATUS) -> winerror::HRESULT {
    const FACILITY_NT_BIT: winerror::HRESULT = 0x1000_0000;
    x | FACILITY_NT_BIT
}
