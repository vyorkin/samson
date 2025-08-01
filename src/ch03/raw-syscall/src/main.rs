fn main() {
    let message = String::from("hello from raw syscall\n");
    syscall(message);
}

#[cfg(target_os = "linux")]
#[inline(never)]
fn syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov rax, 1", // system call 1 means "write" on Linux
            "mov rdi, 1", // file handle - 1 is stdout
            "syscall",    // call kernel, software interrupt
            in("rsi") ptr,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _,
        );
    }
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov rax, 0x2000004", // system call 0x2000004 is write on macos
            "mov rdi, 1",         // file handle 1 is stdout
            "syscall",            // call kernel, syscall interrupt
            in("rsi") msg_ptr,    // address of string to output
            in("rdx") len,         // number of bytes
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(never)]
fn syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        use std::arch::asm;

        asm!(
            "mov x16, 4", // write syscall
            "mov x0, 1",  // stdout
            "svc 0",      // syscall
            in("x1") ptr, // arg1
            in("x2") len, // arg2
            out("x16") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        );
    }
}

#[cfg(target_os = "windows")]
#[inline(never)]
fn syscall(message: String) {
    panic!("We can't. Windows doesn't have a stable syscall ABI")
}
