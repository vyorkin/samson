use std::arch::asm;

#[cfg(target_arch = "x86_64")]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr) };
    res
}

// FIX #11
#[cfg(target_arch = "aarch64")]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    // Take the first 8 bytes (on 64-bit machine) from "{1}"
    // and place that into register represented by "{0}"
    //
    // "[{1}]" means treat the data in that register as a memory address
    //
    // So it fetches the memory location "{1}" and moves it over to "{0}"
    // Note that `ptr` is an address here
    unsafe { asm!("ldr {0}, [{1}]", out(reg) res, in(reg) ptr) };
    // Since we only specify `reg` and not a specific register,
    // we let the compiler choose what registers it wants to use

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dereference_success() {
        let t = 1000;
        let t_ptr: *const usize = &t;
        let x = dereference(t_ptr);
        assert_eq!(1000, x);
    }

    #[test]
    fn test_derefernce_fail() {
        // Pointer to invalid address
        let t_ptr = 99999999999999 as *const usize;
        // Segfauls: (signal: 11, SIGSEGV: invalid memory reference)
        let x = dereference(t_ptr);
        // The error we get is different on different platforms.
        // Surely, the OS is involved somehow
        println!("{x}");
    }
}
