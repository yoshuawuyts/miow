use std::io;
use std::mem;
use std::os::windows::io::*;
use std::time::Duration;

use handle::Handle;
use winapi::*;
use kernel32::*;

pub struct CompletionPort {
    handle: Handle,
}

#[derive(Clone, Copy, Debug)]
pub struct CompletionStatus(OVERLAPPED_ENTRY);

impl CompletionPort {
    pub fn new(threads: u32) -> io::Result<CompletionPort> {
        let ret = unsafe {
            CreateIoCompletionPort(INVALID_HANDLE_VALUE, 0 as *mut _,
                                   0, threads)
        };
        if ret.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(CompletionPort { handle: Handle::new(ret) })
        }
    }

    pub fn add<T: AsRawHandle>(&self, token: usize, t: &T) -> io::Result<()> {
        self._add(token, t.as_raw_handle())
    }

    fn _add(&self, token: usize, handle: HANDLE) -> io::Result<()> {
        let ret = unsafe {
            CreateIoCompletionPort(handle, self.handle.raw(),
                                   token as ULONG_PTR, 0)
        };
        if ret.is_null() {
            Err(io::Error::last_os_error())
        } else {
            debug_assert_eq!(ret, self.handle.raw());
            Ok(())
        }
    }

    pub fn get(&self, timeout: Option<Duration>) -> io::Result<CompletionStatus> {
        let mut bytes = 0;
        let mut token = 0;
        let mut overlapped = 0 as *mut _;
        let timeout = timeout.map(::dur2timeout).unwrap_or(INFINITE);
        let ret = unsafe {
            GetQueuedCompletionStatus(self.handle.raw(),
                                      &mut bytes,
                                      &mut token,
                                      &mut overlapped,
                                      timeout)
        };
        ::cvt(ret).map(|_| {
            CompletionStatus(OVERLAPPED_ENTRY {
                dwNumberOfBytesTransferred: bytes,
                lpCompletionKey: token,
                lpOverlapped: overlapped,
                Internal: 0,
            })
        })
    }

    pub fn get_many<'a>(&self,
                        list: &'a mut [CompletionStatus],
                        timeout: Option<Duration>)
                        -> io::Result<&'a mut [CompletionStatus]>
    {
        debug_assert_eq!(mem::size_of::<CompletionStatus>(),
                         mem::size_of::<OVERLAPPED_ENTRY>());
        let mut removed = 0;
        let timeout = timeout.map(::dur2timeout).unwrap_or(INFINITE);
        let ret = unsafe {
            GetQueuedCompletionStatusEx(self.handle.raw(),
                                        list.as_ptr() as *mut _,
                                        list.len() as ULONG,
                                        &mut removed,
                                        timeout,
                                        FALSE)
        };
        match ::cvt(ret) {
            Ok(_) => Ok(&mut list[..removed as usize]),
            Err(e) => Err(e),
        }
    }

    pub fn post(&self, status: CompletionStatus) -> io::Result<()> {
        let ret = unsafe {
            PostQueuedCompletionStatus(self.handle.raw(),
                                       status.0.dwNumberOfBytesTransferred,
                                       status.0.lpCompletionKey,
                                       status.0.lpOverlapped)
        };
        ::cvt(ret).map(|_| ())
    }
}

impl AsRawHandle for CompletionPort {
    fn as_raw_handle(&self) -> HANDLE {
        self.handle.raw()
    }
}

impl FromRawHandle for CompletionPort {
    unsafe fn from_raw_handle(handle: HANDLE) -> CompletionPort {
        CompletionPort { handle: Handle::new(handle) }
    }
}

#[cfg(feature = "unstable")]
impl IntoRawHandle for CompletionPort {
    fn into_raw_handle(self) -> HANDLE {
        self.handle.into_raw()
    }
}

impl CompletionStatus {
    pub fn new(bytes: u32, token: usize, overlapped: *mut OVERLAPPED)
               -> CompletionStatus {
        CompletionStatus(OVERLAPPED_ENTRY {
            dwNumberOfBytesTransferred: bytes,
            lpCompletionKey: token as ULONG_PTR,
            lpOverlapped: overlapped,
            Internal: 0,
        })
    }

    pub fn zero() -> CompletionStatus {
        CompletionStatus::new(0, 0, 0 as *mut _)
    }

    pub fn bytes_transferred(&self) -> u32 {
        self.0.dwNumberOfBytesTransferred
    }

    pub fn token(&self) -> usize {
        self.0.lpCompletionKey as usize
    }

    pub fn overlapped(&self) -> *mut OVERLAPPED {
        self.0.lpOverlapped
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::time::Duration;
    use winapi::*;

    use {CompletionPort, CompletionStatus};

    #[test]
    fn token_right_size() {
        assert_eq!(mem::size_of::<usize>(), mem::size_of::<ULONG_PTR>());
    }

    #[test]
    fn timeout() {
        let c = CompletionPort::new(1).unwrap();
        let err = c.get(Some(Duration::new(0, 1_000_000))).unwrap_err();
        assert_eq!(err.raw_os_error(), Some(WAIT_TIMEOUT as i32));
    }

    #[test]
    fn get() {
        let c = CompletionPort::new(1).unwrap();
        c.post(CompletionStatus::new(1, 2, 3 as *mut _)).unwrap();
        let s = c.get(None).unwrap();
        assert_eq!(s.bytes_transferred(), 1);
        assert_eq!(s.token(), 2);
        assert_eq!(s.overlapped(), 3 as *mut _);
    }

    #[test]
    fn get_many() {
        let c = CompletionPort::new(1).unwrap();

        c.post(CompletionStatus::new(1, 2, 3 as *mut _)).unwrap();
        c.post(CompletionStatus::new(4, 5, 6 as *mut _)).unwrap();

        let mut s = vec![CompletionStatus::zero(); 4];
        {
            let s = c.get_many(&mut s, None).unwrap();
            assert_eq!(s.len(), 2);
            assert_eq!(s[0].bytes_transferred(), 1);
            assert_eq!(s[0].token(), 2);
            assert_eq!(s[0].overlapped(), 3 as *mut _);
            assert_eq!(s[1].bytes_transferred(), 4);
            assert_eq!(s[1].token(), 5);
            assert_eq!(s[1].overlapped(), 6 as *mut _);
        }
        assert_eq!(s[2].bytes_transferred(), 0);
        assert_eq!(s[2].token(), 0);
        assert_eq!(s[2].overlapped(), 0 as *mut _);
    }
}
