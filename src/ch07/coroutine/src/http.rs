use crate::future::{Future, PollState};
use std::io::{ErrorKind, Read, Write};

fn get_request(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n
             HOST localhost\r\n
             Connection: close\r\n
             \r\n"
    )
}

pub struct Http;

impl Http {
    pub fn get(path: &'static str) -> impl Future<Output = String> {
        HttpGetFuture::new(path)
    }
}

struct HttpGetFuture {
    stream: Option<mio::net::TcpStream>,
    buffer: Vec<u8>,
    path: String,
}

impl HttpGetFuture {
    fn new(path: &'static str) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            path: path.to_string(),
        }
    }

    fn write_request(&mut self) {
        let stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        stream.set_nonblocking(true).unwrap();
        let mut stream = mio::net::TcpStream::from_std(stream);
        let request_string = get_request(&self.path);
        let request_bytes = request_string.as_bytes();
        stream.write_all(request_bytes).unwrap();
        self.stream = Some(stream)
    }
}
impl Future for HttpGetFuture {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        if self.stream.is_none() {
            println!("first poll, start operation");
            self.write_request();
            return PollState::NotReady;
        }

        let mut buf = vec![0u8; 4096];
        let tcp_stream = self.stream.as_mut().unwrap();
        loop {
            match tcp_stream.read(&mut buf) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    break PollState::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buf[0..n]);
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    // 1) data isn't ready yet
                    // 2) there is more data but we haven't received it yet
                    //
                    // break and wait for the next call to `poll()`
                    break PollState::NotReady; // Pending
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    // interrupted by signal - try reading once more
                    continue;
                }
                Err(e) => {
                    panic!("{e:?}")
                }
            }
        }
    }
}
