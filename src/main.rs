extern crate wio;
extern crate winapi;
extern crate ws2_32;

use std::io;
use std::io::prelude::*;
use std::os::windows::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;

use wio::CompletionPort;
use winapi::*;

fn main() {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = l.local_addr().unwrap();
    let t = thread::spawn(move || {
        let mut a = l.accept().unwrap().0;
        a.write_all(&[1, 2, 3, 4]).unwrap();
    });

    let mut b = [0u8; 10];
    let mut a = WSABUF {
        len: b.len() as u_long,
        buf: b.as_mut_ptr() as *mut _,
    };
    let mut b2 = [0u8; 256 * 1024];
    let mut a2 = WSABUF {
        len: b2.len() as u_long,
        buf: b2.as_mut_ptr() as *mut _,
    };
    let mut overlapped = WSAOVERLAPPED {
        Internal: 0,
        InternalHigh: 0,
        Offset: 0,
        OffsetHigh: 0,
        hEvent: 0 as *mut _,
    };
    let mut overlapped2 = WSAOVERLAPPED {
        Internal: 0,
        InternalHigh: 0,
        Offset: 0,
        OffsetHigh: 0,
        hEvent: 0 as *mut _,
    };
    let mut flags = 0;
    let s = TcpStream::connect(&addr).unwrap();
    let cp = CompletionPort::new(1).unwrap();
    cp.add_socket(1, &s).unwrap();
    unsafe {
        let r = ws2_32::WSARecv(s.as_raw_socket(),
                                &mut a,
                                1,
                                0 as *mut _,
                                &mut flags,
                                &mut overlapped,
                                None);
        let err = io::Error::last_os_error();
        println!("{} {}", r, err);

        let r = ws2_32::WSASend(s.as_raw_socket(),
                                &mut a2,
                                1,
                                0 as *mut _,
                                0,
                                &mut overlapped2,
                                None);
        let err = io::Error::last_os_error();
        println!("{} {}", r, err);

        let mut transferred = 0;
        let mut my_flags = 0;
        println!("waiting for overlap result");
        let r = ws2_32::WSAGetOverlappedResult(s.as_raw_socket(),
                                               &mut overlapped,
                                               &mut transferred,
                                               TRUE,
                                               &mut my_flags);
        let err = io::Error::last_os_error();
        println!("got overlap result");
        println!("{} {} {} {}", r, err, transferred, my_flags);
    }

    println!("{:p}", &overlapped);
    println!("{:p}", &overlapped2);

    let status = cp.get(None).unwrap();
    println!("{:?}", status);
    println!("{:?}", &b[..]);

    let status = cp.get(None).unwrap();
    println!("{:?}", status);
    println!("{:?}", &b[..]);


    t.join().unwrap();
}
