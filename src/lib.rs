#![cfg_attr(feature = "unstable", feature(into_raw_os))]
#![cfg(windows)]

extern crate kernel32;
extern crate winapi;

use std::time::Duration;
use std::io;
use winapi::*;

mod handle;
mod iocp;

pub use iocp::{CompletionPort, CompletionStatus};

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
