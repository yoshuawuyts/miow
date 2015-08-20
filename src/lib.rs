//! A zero overhead Windows I/O library

#![cfg_attr(feature = "unstable", feature(into_raw_os))]
#![cfg(windows)]
#![deny(missing_docs)]
#![doc(html_root_url = "http://alexcrichton.com/wio")]

extern crate kernel32;
extern crate libc;
extern crate net2;
extern crate winapi;
extern crate ws2_32;

use std::time::Duration;
use std::io;
use winapi::*;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
    })
}

mod handle;
mod iocp;
mod net;

pub use iocp::{CompletionPort, CompletionStatus};
pub use net::{TcpStreamExt, UdpSocketExt, SocketAddrBuf, TcpBuilderExt};
pub use net::{TcpListenerExt, AcceptAddrsBuf, AcceptAddrs};

fn dur2timeout(dur: Duration) -> DWORD {
    // Note that a duration is a (u64, u32) (seconds, nanoseconds) pair, and the
    // timeouts in windows APIs are typically u32 milliseconds. To translate, we
    // have two pieces to take care of:
    //
    // * Nanosecond precision is rounded up
    // * Greater than u32::MAX milliseconds (50 days) is rounded up to INFINITE
    //   (never time out).
    dur.as_secs().checked_mul(1000).and_then(|ms| {
        ms.checked_add((dur.subsec_nanos() as u64) / 1_000_000)
    }).and_then(|ms| {
        ms.checked_add(if dur.subsec_nanos() % 1_000_000 > 0 {1} else {0})
    }).map(|ms| {
        if ms > <DWORD>::max_value() as u64 {
            INFINITE
        } else {
            ms as DWORD
        }
    }).unwrap_or(INFINITE)
}

fn cvt(i: BOOL) -> io::Result<BOOL> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}
