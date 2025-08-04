use core::arch::asm;

const SSIZE: isize = 48;

/// Represents CPU state.
#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    /// Stack pointer.
    rsp: u64,
}

fn hello() -> ! {
    println!("awake");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
    );
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8, SSIZE as usize];

    unsafe {}
}
