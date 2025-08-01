mod ffi;
mod poll;

use ffi::Event;
use poll::Poll;

fn main() -> std::io::Result<()> {
    let mut poll = Poll::new()?;

    Ok(())
}
