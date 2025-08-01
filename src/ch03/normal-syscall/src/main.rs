#[cfg(target_family = "unix")]
use std::io;

fn main() {
    let message = "hello from normal syscall\n";
    let message = String::from(message);
    syscall(message).unwrap();
}

// Linux/macOS

#[cfg(target_family = "unix")]
#[link(name = "c")]
unsafe extern "C" {
    unsafe fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

#[cfg(target_family = "unix")]
fn syscall(message: String) -> io::Result<()> {
    let ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(1, ptr, len) };

    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(target_family = "windows")]
#[link(name = "kernel32")]
extern "system" {
    /// https://docs.microsoft.com/en-us/windows/console/getstdhandle
    fn GetStdHandle(nStdHandle: i32) -> i32;
    /// https://docs.microsoft.com/en-us/windows/console/writeconsole
    fn WriteConsoleW(
        hConsoleOutput: i32,
        lpBuffer: *const u16,
        numberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *const std::ffi::c_void,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn syscall(message: String) -> io::Result<()> {
    // let's convert our utf-8 to a format windows understands
    let msg: Vec<u16> = message.encode_utf16().collect();
    let msg_ptr = msg.as_ptr();
    let len = msg.len() as u32;

    let mut output: u32 = 0;
    let handle = unsafe { GetStdHandle(-11) };
    if handle == -1 {
        return Err(io::Error::last_os_error());
    }

    let res = unsafe { WriteConsoleW(handle, msg_ptr, len, &mut output, std::ptr::null()) };
    if res == 0 {
        return Err(io::Error::last_os_error());
    }

    // Just assert that the output variable we wrote all the bytes we expected
    // and panic if we didn't
    assert_eq!(output, len);
    Ok(())
}
