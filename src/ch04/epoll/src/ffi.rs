// Indicates that we want to perform an `add` operation
// (start watching for a notification).
//
// See the comment for `epoll_ctl` below.
pub const EPOLL_CTL_ADD: i32 = 1;

// Bitflags:

// Interest in `read` operation on the file handle.
pub const EPOLLIN: i32 = 0x1;
// Set epoll to "edge-triggered mode" (see below).
pub const EPOLLET: i32 = 1 << 31;

// epoll watch modes:
//
// 1. level-triggered mode
//    We get notified of the same event until the buffer is drained.
//
// 2. edge-triggered mode
//    We get notified only once: when buffer is changed from
//    having no data to having some data.
//    Note that if you don't drain the buffer you will not receive
//    the notification again.
//
// ! mio uses epoll is an edge-triggered mode !

#[derive(Debug)]
#[repr(C)]
#[cfg_attr(target_arch = "x86_64", repr(packed))]
pub struct Event {
    // Bitmask represents:
    // - what events we're interested in
    // OR
    // - what events occured.
    pub(crate) events: u32,

    // Event id.
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}

// Syscalls.
#[link(name = "c")]
unsafe extern "C" {
    // Creates an epoll queue.
    //
    // The `size` isn't used and kept for historical reasons.
    // The `size` argument should have any value larger than 0.
    // See: https://man7.org/linux/man-pages/man2/epoll_create.2.html
    pub unsafe fn epoll_create(size: i32) -> i32;

    // Closes file the descriptor when get when we create an epoll to release the resources.
    //
    // See: https://man7.org/linux/man-pages/man2/close.2.html
    pub unsafe fn close(fd: i32) -> i32;

    // Epoll control interface.
    // We use it to register interest in events on a source.
    // Supports 3 operations: add, modify, delete.
    //
    // `epdf` - Epoll file descriptor we want to operations on.
    // `op` - Operation we want to perform.
    // `events` - What kind of events we want to be notified of, how and when we get notified.
    //
    // See: https://man7.org/linux/man-pages/man2/epoll_ctl.2.html
    pub unsafe fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;

    // Blocks the current thread and waits one of two things happen:
    // 1. Event has occured
    // 2. Timeout
    //
    // `epdf` - Epoll file descriptor.
    // `events` - What events did occur.
    // `maxevents` - How many events we have reserved space for in the `events` array.
    // `timeout` - How long to wait for events before the OS kernel will wake us up again.
    //
    // See: https://man7.org/linux/man-pages/man2/epoll_wait.2.html
    pub unsafe fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}
