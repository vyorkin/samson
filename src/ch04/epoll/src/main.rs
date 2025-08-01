mod ffi;
mod poll;

use std::{
    collections::HashSet,
    env,
    io::{self, Read, Write},
    net::TcpStream,
};

use ffi::Event;
use poll::Poll;

fn main() -> std::io::Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams: Vec<TcpStream> = vec![];

    let base_url = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost"));

    let addr = format!("{base_url}:8080");

    for i in 0..n_events {
        let delay = (n_events - 1) * 1000;

        let url_path = format!("/{delay}/request-{i}");
        let request = format!(
            "GET {url_path} HTTP/1.1\r\n
                 Host: localhost\r\n
                 Connection: close\r\n
                 \r\n"
        );

        let mut stream = TcpStream::connect(&addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;
        let interests = ffi::EPOLLIN | ffi::EPOLLET;
        poll.registry().register(&stream, i, interests)?;

        streams.push(stream);
    }

    let mut handled_ids: HashSet<usize> = HashSet::new();
    let mut n_handled_events = 0;

    while n_handled_events < n_events {
        let mut events: Vec<Event> = Vec::with_capacity(10);

        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("timeout\n");
            continue;
        }

        n_handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }

    println!("finished");

    Ok(())
}

fn handle_events(
    events: &[Event],
    streams: &mut [TcpStream],
    handled_ids: &mut HashSet<usize>,
) -> std::io::Result<usize> {
    let mut n_handled_events = 0;
    for event in events {
        let event_id = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[event_id].read(&mut data) {
                Ok(0) => {
                    if !handled_ids.insert(event_id) {
                        break;
                    }
                    n_handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("received: {event:?}");
                    println!("{txt}")
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(n_handled_events)
}
