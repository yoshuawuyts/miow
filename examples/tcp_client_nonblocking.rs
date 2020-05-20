use std::io;
use std::net::TcpStream;

use miow::iocp::CompletionPort;
use miow::net::TcpStreamExt;
use miow::Overlapped;

// Open a TCP server on port 8080 and make it serve a file. Example:
// $ nc -l -p 8080 < README.md
fn main() -> io::Result<()> {
    let cp = CompletionPort::new(1)?;
    let stream = TcpStream::connect("localhost:8080")?;
    stream.set_nonblocking(true)?;
    cp.add_socket(1, &stream)?;

    let mut buf = vec![0; 4028];
    let a = Overlapped::zero();
    unsafe {
        stream.read_overlapped(&mut buf, a.raw())?;
    }
    let status = cp.get(None)?;
    let read = status.bytes_transferred() as usize;

    println!("> {:?}", String::from_utf8(buf[0..read].to_vec()).unwrap());
    Ok(())
}
