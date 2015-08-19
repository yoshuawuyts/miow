use std::io;
use std::mem;
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::net::{SocketAddrV4, Ipv4Addr, SocketAddrV6, Ipv6Addr};
use std::os::windows::prelude::*;

use winapi::*;
use ws2_32::*;

#[doc(hidden)]
trait NetInt {
    fn from_be(i: Self) -> Self;
    fn to_be(&self) -> Self;
}
macro_rules! doit {
    ($($t:ident)*) => ($(impl NetInt for $t {
        fn from_be(i: Self) -> Self { <$t>::from_be(i) }
        fn to_be(&self) -> Self { <$t>::to_be(*self) }
    })*)
}
doit! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

// fn hton<I: NetInt>(i: I) -> I { i.to_be() }
fn ntoh<I: NetInt>(i: I) -> I { I::from_be(i) }

fn cvt(i: c_int) -> io::Result<bool> {
    if i == SOCKET_ERROR {
        let err = unsafe { WSAGetLastError() };
        if err == WSA_IO_PENDING as i32 {
            Ok(false)
        } else {
            Err(io::Error::from_raw_os_error(err))
        }
    } else {
        Ok(true)
    }
}

/// A type to represent a buffer in which a socket address will be stored.
///
/// This type is used with the `recv_from_overlapped` function on the
/// `UdpSocketExt` trait to provide space for the overlapped I/O operation to
/// fill in the address upon completion.
#[derive(Clone, Copy)]
pub struct SocketAddrBuf {
    buf: SOCKADDR_STORAGE,
    len: c_int,
}

/// Additional methods for the `TcpStream` type in the standard library.
pub trait TcpStreamExt {
    /// Execute an overlapped read I/O operation on this TCP stream.
    ///
    /// This function will issue an overlapped I/O read (via `WSARecv`) on this
    /// socket. The provided buffer will be filled in when the operation
    /// completes and the given `WSAOVERLAPPED` instance is used to track the
    /// overlapped operation.
    ///
    /// If the operation succeeds, `Ok(true)` is returned. If the operation
    /// returns an error indicating that the I/O is currently pending,
    /// `Ok(false)` is returned. Otherwise, the error associated with the
    /// operation is returned and no overlapped operation is enqueued.
    ///
    /// The number of bytes read will be returned as part of the completion
    /// notification when the I/O finishes.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because the kernel requires that the `buf` and
    /// `overlapped` pointers are valid until the end of the I/O operation. The
    /// kernel also requires that `overlapped` is unique for this I/O operation
    /// and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that these two input
    /// pointers are valid until the I/O operation is completed, typically via
    /// completion ports and waiting to receive the completion notification on
    /// the port.
    unsafe fn read_overlapped(&self,
                              buf: &mut [u8],
                              overlapped: &mut WSAOVERLAPPED) -> io::Result<bool>;

    /// Execute an overlapped write I/O operation on this TCP stream.
    ///
    /// This function will issue an overlapped I/O write (via `WSASend`) on this
    /// socket. The provided buffer will be written when the operation completes
    /// and the given `WSAOVERLAPPED` instance is used to track the overlapped
    /// operation.
    ///
    /// If the operation succeeds, `Ok(true)` is returned. If the operation
    /// returns an error indicating that the I/O is currently pending,
    /// `Ok(false)` is returned. Otherwise, the error associated with the
    /// operation is returned and no overlapped operation is enqueued.
    ///
    /// The number of bytes written will be returned as part of the completion
    /// notification when the I/O finishes.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because the kernel requires that the `buf` and
    /// `overlapped` pointers are valid until the end of the I/O operation. The
    /// kernel also requires that `overlapped` is unique for this I/O operation
    /// and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that these two input
    /// pointers are valid until the I/O operation is completed, typically via
    /// completion ports and waiting to receive the completion notification on
    /// the port.
    unsafe fn write_overlapped(&self,
                               buf: &[u8],
                               overlapped: &mut WSAOVERLAPPED) -> io::Result<bool>;
}

impl TcpStreamExt for TcpStream {
    unsafe fn read_overlapped(&self, buf: &mut [u8],
                              overlapped: &mut OVERLAPPED) -> io::Result<bool> {
        let mut buf = WSABUF {
            len: buf.len() as u_long,
            buf: buf.as_mut_ptr() as *mut _,
        };
        let mut flags = 0;
        let r = WSARecv(self.as_raw_socket(), &mut buf, 1,
                        0 as *mut _, &mut flags, overlapped, None);
        cvt(r)
    }

    unsafe fn write_overlapped(&self, buf: &[u8],
                               overlapped: &mut OVERLAPPED) -> io::Result<bool> {
        let mut buf = WSABUF {
            len: buf.len() as u_long,
            buf: buf.as_ptr() as *mut _,
        };
        let r = WSASend(self.as_raw_socket(), &mut buf, 1,
                        0 as *mut _, 0, overlapped, None);
        cvt(r)
    }
}

/// Additional methods for the `UdpSocket` type in the standard library.
pub trait UdpSocketExt {
    /// Execute an overlapped receive I/O operation on this UDP socket.
    ///
    /// This function will issue an overlapped I/O read (via `WSARecvFrom`) on
    /// this socket. The provided buffer will be filled in when the operation
    /// completes, the source from where the data came from will be written to
    /// `addr`, and the given `WSAOVERLAPPED` instance is used to track the
    /// overlapped operation.
    ///
    /// If the operation succeeds, `Ok(true)` is returned. If the operation
    /// returns an error indicating that the I/O is currently pending,
    /// `Ok(false)` is returned. Otherwise, the error associated with the
    /// operation is returned and no overlapped operation is enqueued.
    ///
    /// The number of bytes read will be returned as part of the completion
    /// notification when the I/O finishes.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because the kernel requires that the `buf`,
    /// `addr`, and `overlapped` pointers are valid until the end of the I/O
    /// operation. The kernel also requires that `overlapped` is unique for this
    /// I/O operation and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that these two input
    /// pointers are valid until the I/O operation is completed, typically via
    /// completion ports and waiting to receive the completion notification on
    /// the port.
    unsafe fn recv_from_overlapped(&self,
                                   buf: &mut [u8],
                                   addr: &mut SocketAddrBuf,
                                   overlapped: &mut WSAOVERLAPPED)
                                   -> io::Result<bool>;

    /// Execute an overlapped send I/O operation on this UDP socket.
    ///
    /// This function will issue an overlapped I/O write (via `WSASendTo`) on
    /// this socket to the address specified by `addr`. The provided buffer will
    /// be written when the operation completes and the given `WSAOVERLAPPED`
    /// instance is used to track the overlapped operation.
    ///
    /// If the operation succeeds, `Ok(true)` is returned. If the operation
    /// returns an error indicating that the I/O is currently pending,
    /// `Ok(false)` is returned. Otherwise, the error associated with the
    /// operation is returned and no overlapped operation is enqueued.
    ///
    /// The number of bytes written will be returned as part of the completion
    /// notification when the I/O finishes.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because the kernel requires that the `buf` and
    /// `overlapped` pointers are valid until the end of the I/O operation. The
    /// kernel also requires that `overlapped` is unique for this I/O operation
    /// and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that these two input
    /// pointers are valid until the I/O operation is completed, typically via
    /// completion ports and waiting to receive the completion notification on
    /// the port.
    unsafe fn send_to_overlapped(&self,
                                 buf: &[u8],
                                 addr: &SocketAddr,
                                 overlapped: &mut WSAOVERLAPPED)
                                 -> io::Result<bool>;
}

impl UdpSocketExt for UdpSocket {
    unsafe fn recv_from_overlapped(&self,
                                   buf: &mut [u8],
                                   addr: &mut SocketAddrBuf,
                                   overlapped: &mut WSAOVERLAPPED)
                                   -> io::Result<bool> {
        let mut buf = WSABUF {
            len: buf.len() as u_long,
            buf: buf.as_mut_ptr() as *mut _,
        };
        let mut flags = 0;
        let r = WSARecvFrom(self.as_raw_socket(), &mut buf, 1,
                            0 as *mut _, &mut flags,
                            &mut addr.buf as *mut _ as *mut _,
                            &mut addr.len,
                            overlapped, None);
        cvt(r)
    }

    unsafe fn send_to_overlapped(&self,
                                 buf: &[u8],
                                 addr: &SocketAddr,
                                 overlapped: &mut WSAOVERLAPPED)
                                 -> io::Result<bool> {
        use libc::{sockaddr_in, sockaddr_in6};

        let (addr_buf, addr_len) = match *addr {
            SocketAddr::V4(ref a) => {
                (a as *const _ as *mut _, mem::size_of::<sockaddr_in>())
            }
            SocketAddr::V6(ref a) => {
                (a as *const _ as *mut _, mem::size_of::<sockaddr_in6>())
            }
        };

        let mut buf = WSABUF {
            len: buf.len() as u_long,
            buf: buf.as_ptr() as *mut _,
        };
        let r = WSASendTo(self.as_raw_socket(), &mut buf, 1,
                          0 as *mut _, 0,
                          addr_buf,
                          addr_len as c_int,
                          overlapped, None);
        cvt(r)
    }
}

impl SocketAddrBuf {
    /// Creates a new blank socket address buffer.
    ///
    /// This should be used before a call to `recv_from_overlapped` overlapped
    /// to create an instance to pass down.
    pub fn new() -> SocketAddrBuf {
        SocketAddrBuf {
            buf: unsafe { mem::zeroed() },
            len: mem::size_of::<SOCKADDR_STORAGE>() as c_int,
        }
    }

    /// Parses this buffer to return a standard socket address.
    ///
    /// This function should be called after the buffer has been filled in with
    /// a call to `recv_from_overlapped` being completed. It will interpret the
    /// address filled in and return the standard socket address type.
    ///
    /// If an error is encountered then `None` is returned.
    pub fn to_socket_addr(&self) -> Option<SocketAddr> {
        use libc::{sockaddr_in, sockaddr_in6, sockaddr_storage, sa_family_t};
        if (self.len as usize) < mem::size_of::<sa_family_t>() {
            return None
        }
        let b = unsafe {
            &*(&self.buf as *const _ as *const sockaddr_storage)
        };
        match b.ss_family as i32 {
            AF_INET if self.len as usize >= mem::size_of::<sockaddr_in>() => {
                let b = unsafe {
                    &*(&self.buf as *const _ as *const sockaddr_in)
                };
                let ip = ntoh(b.sin_addr.s_addr);
                let ip = Ipv4Addr::new((ip >> 24) as u8,
                                       (ip >> 16) as u8,
                                       (ip >>  8) as u8,
                                       (ip >>  0) as u8);
                Some(SocketAddr::V4(SocketAddrV4::new(ip, ntoh(b.sin_port))))
            }
            AF_INET6 if self.len as usize >= mem::size_of::<sockaddr_in6>() => {
                let b = unsafe {
                    &*(&self.buf as *const _ as *const sockaddr_in6)
                };
                let ip = Ipv6Addr::new(ntoh(b.sin6_addr.s6_addr[0]),
                                       ntoh(b.sin6_addr.s6_addr[1]),
                                       ntoh(b.sin6_addr.s6_addr[2]),
                                       ntoh(b.sin6_addr.s6_addr[3]),
                                       ntoh(b.sin6_addr.s6_addr[4]),
                                       ntoh(b.sin6_addr.s6_addr[5]),
                                       ntoh(b.sin6_addr.s6_addr[6]),
                                       ntoh(b.sin6_addr.s6_addr[7]));
                let addr = SocketAddrV6::new(ip, ntoh(b.sin6_port),
                                             ntoh(b.sin6_flowinfo),
                                             ntoh(b.sin6_scope_id));
                Some(SocketAddr::V6(addr))
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{TcpListener, UdpSocket, TcpStream, SocketAddr};
    use std::thread;
    use std::io::prelude::*;
    use winapi::*;

    use {CompletionPort, TcpStreamExt, UdpSocketExt, SocketAddrBuf};

    fn overlapped() -> WSAOVERLAPPED {
        WSAOVERLAPPED {
            Internal: 0,
            InternalHigh: 0,
            Offset: 0,
            OffsetHigh: 0,
            hEvent: 0 as *mut _,
        }
    }

    fn each_ip(f: &mut FnMut(SocketAddr)) {
        f("127.0.0.1:0".parse().unwrap());
        f("[::1]:0".parse().unwrap());
    }

    #[test]
    fn tcp_read() {
        each_ip(&mut |addr| {
            let l = TcpListener::bind(addr).unwrap();
            let addr = l.local_addr().unwrap();
            let t = thread::spawn(move || {
                let mut a = l.accept().unwrap().0;
                a.write_all(&[1, 2, 3]).unwrap();
            });

            let cp = CompletionPort::new(1).unwrap();
            let s = TcpStream::connect(addr).unwrap();
            cp.add_socket(1, &s).unwrap();

            let mut b = [0; 10];
            let mut a = overlapped();
            unsafe {
                s.read_overlapped(&mut b, &mut a).unwrap();
            }
            let status = cp.get(None).unwrap();
            assert_eq!(status.bytes_transferred(), 3);
            assert_eq!(status.token(), 1);
            assert_eq!(status.overlapped(), &mut a as *mut _);
            assert_eq!(&b[0..3], &[1, 2, 3]);

            t.join().unwrap();
        })
    }

    #[test]
    fn tcp_write() {
        each_ip(&mut |addr| {
            let l = TcpListener::bind(addr).unwrap();
            let addr = l.local_addr().unwrap();
            let t = thread::spawn(move || {
                let mut a = l.accept().unwrap().0;
                let mut b = [0; 10];
                let n = a.read(&mut b).unwrap();
                assert_eq!(n, 3);
                assert_eq!(&b[0..3], &[1, 2, 3]);
            });

            let cp = CompletionPort::new(1).unwrap();
            let s = TcpStream::connect(addr).unwrap();
            cp.add_socket(1, &s).unwrap();

            let b = [1, 2, 3];
            let mut a = overlapped();
            unsafe {
                s.write_overlapped(&b, &mut a).unwrap();
            }
            let status = cp.get(None).unwrap();
            assert_eq!(status.bytes_transferred(), 3);
            assert_eq!(status.token(), 1);
            assert_eq!(status.overlapped(), &mut a as *mut _);

            t.join().unwrap();
        })
    }

    #[test]
    fn udp_recv_from() {
        each_ip(&mut |addr| {
            let a = UdpSocket::bind(addr).unwrap();
            let b = UdpSocket::bind(addr).unwrap();
            let a_addr = a.local_addr().unwrap();
            let b_addr = b.local_addr().unwrap();
            let t = thread::spawn(move || {
                a.send_to(&[1, 2, 3], b_addr).unwrap();
            });

            let cp = CompletionPort::new(1).unwrap();
            cp.add_socket(1, &b).unwrap();

            let mut buf = [0; 10];
            let mut a = overlapped();
            let mut addr = SocketAddrBuf::new();
            unsafe {
                b.recv_from_overlapped(&mut buf, &mut addr, &mut a).unwrap();
            }
            let status = cp.get(None).unwrap();
            assert_eq!(status.bytes_transferred(), 3);
            assert_eq!(status.token(), 1);
            assert_eq!(status.overlapped(), &mut a as *mut _);
            assert_eq!(&buf[..3], &[1, 2, 3]);
            assert_eq!(addr.to_socket_addr(), Some(a_addr));

            t.join().unwrap();
        })
    }

    #[test]
    fn udp_send_to() {
        each_ip(&mut |addr| {
            let a = UdpSocket::bind(addr).unwrap();
            let b = UdpSocket::bind(addr).unwrap();
            let a_addr = a.local_addr().unwrap();
            let b_addr = b.local_addr().unwrap();
            let t = thread::spawn(move || {
                let mut b = [0; 100];
                let (n, addr) = a.recv_from(&mut b).unwrap();
                assert_eq!(n, 3);
                assert_eq!(addr, b_addr);
                assert_eq!(&b[..3], &[1, 2, 3]);
            });

            let cp = CompletionPort::new(1).unwrap();
            cp.add_socket(1, &b).unwrap();

            let mut a = overlapped();
            unsafe {
                b.send_to_overlapped(&[1, 2, 3], &a_addr, &mut a).unwrap();
            }
            let status = cp.get(None).unwrap();
            assert_eq!(status.bytes_transferred(), 3);
            assert_eq!(status.token(), 1);
            assert_eq!(status.overlapped(), &mut a as *mut _);

            t.join().unwrap();
        })
    }
}
