use std::{thread, time::Duration};

use crate::{
    future::{Future, PollState},
    http::Http,
};

mod future;
mod http;

// This state machine would be similar to the one created by:
// async fn async_main() {
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").await;
//     println!("{txt}");
//     let txt = http::Http::get("/400/HelloAsyncAwait").await;
//     println!("{txt}");
// }

struct Coroutine {
    state: State,
}

impl Coroutine {
    pub fn new() -> Self {
        Self {
            state: State::Start,
        }
    }
}

impl Future for Coroutine {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("start");
                    let future = Box::new(Http::get("/600/hello1"));
                    self.state = State::Wait1(future);
                }
                State::Wait1(ref mut future) => match future.poll() {
                    PollState::Ready(txt) => {
                        println!("future1 ready: {txt}");
                        let future = Box::new(Http::get("/400/hello2"));
                        self.state = State::Wait2(future);
                    }
                    PollState::NotReady => break PollState::NotReady,
                },
                State::Wait2(ref mut future) => match future.poll() {
                    PollState::Ready(txt) => {
                        println!("future2 ready: {txt}");
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }
                    PollState::NotReady => break PollState::NotReady,
                },
                State::Resolved => {
                    panic!("polled a resolved future");
                }
            }
        }
    }
}

enum State {
    // The Coroutine has been created but hasn't been polled yet.
    Start,
    // First call to `Http::get`.
    Wait1(Box<dyn Future<Output = String>>),
    // Second call to `Http::get`.
    Wait2(Box<dyn Future<Output = String>>),
    // No more work to do.
    Resolved,
}

fn async_main() -> impl Future<Output = ()> {
    Coroutine::new()
}

fn main() {
    let mut future = async_main();

    while let PollState::NotReady = future.poll() {
        println!("schedule other tasks");
        thread::sleep(Duration::from_millis(100));
    }
}
