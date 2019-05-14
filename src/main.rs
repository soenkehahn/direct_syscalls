#![feature(asm)]

use std::ffi::CString;

fn main() -> Result<(), Box<std::error::Error>> {
    let fd: u64 = 1; // fd 1 is stdout
    let string: String = "huhu\n".to_string();
    let count: usize = string.len();
    let c_string = CString::new(string)?;
    write_syscall(fd, c_string.as_ptr(), count);
    Ok(())
}

fn write_syscall(fd: u64, c_string_ptr: *const i8, count: usize) {
    let syscall_number: i64 = 1; // syscall 1 is write
    unsafe {
        asm!(
            "syscall"
            :
            : "{rax}"(syscall_number), "{rdi}"(fd), "{rsi}"(c_string_ptr), "{rdx}"(count)
            : "rcx", "r11", "memory"
            : "volatile"
        );
    }
}
