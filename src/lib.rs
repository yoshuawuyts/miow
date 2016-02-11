//! A zero overhead Windows I/O library

#![cfg(windows)]
#![deny(missing_docs)]
#![allow(bad_style)]
#![doc(html_root_url = "http://alexcrichton.com/miow")]

extern crate kernel32;
extern crate net2;
extern crate winapi;
extern crate ws2_32;

#[cfg(test)] extern crate rand;

use std::io;
use winapi::*;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
    })
}

mod handle;
mod overlapped;

pub mod iocp;
pub mod net;
pub mod pipe;

pub use overlapped::Overlapped;

fn cvt(i: BOOL) -> io::Result<BOOL> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}
