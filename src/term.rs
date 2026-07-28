// Linux-only. No libc: talks to the kernel directly via raw syscalls
// (inline asm), instead of linking tcgetattr/tcsetattr/ioctl from C.
use std::arch::asm;
use std::io::{self, Write};

#[cfg(target_arch = "x86_64")]
mod sysno {
    pub const READ: usize = 0;
    pub const WRITE: usize = 1;
    pub const IOCTL: usize = 16;
}

#[cfg(target_arch = "aarch64")]
mod sysno {
    pub const READ: usize = 63;
    pub const WRITE: usize = 64;
    pub const IOCTL: usize = 29;
}

// ioctl request numbers, from asm-generic/ioctls.h (same value on x86_64 and aarch64).
const TCGETS: usize = 0x5401;
const TCSETS: usize = 0x5402;

const STDIN_FD: usize = 0;
const ICANON: u32 = 0o0000002;
const ECHO: u32 = 0o0000010;
const VMIN: usize = 6;
const VTIME: usize = 5;

// This is the *kernel* struct termios (asm-generic/termbits.h), which is
// laid out differently from glibc's struct termios: NCCS is 19, not 32,
// and there are no c_ispeed/c_ospeed fields.
#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 19],
}

#[inline(always)]
unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
    let ret: isize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as isize => ret,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            out("rcx") _,
            out("r11") _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x8") n,
            options(nostack)
        );
    }
    ret
}

fn check(ret: isize) -> io::Result<usize> {
    if ret < 0 {
        Err(io::Error::from_raw_os_error(-ret as i32))
    } else {
        Ok(ret as usize)
    }
}

fn sys_ioctl_get(fd: usize) -> io::Result<Termios> {
    let mut t: Termios = unsafe { std::mem::zeroed() };
    check(unsafe { syscall3(sysno::IOCTL, fd, TCGETS, &mut t as *mut _ as usize) })?;
    Ok(t)
}

fn sys_ioctl_set(fd: usize, t: &Termios) -> io::Result<()> {
    check(unsafe { syscall3(sysno::IOCTL, fd, TCSETS, t as *const _ as usize) })?;
    Ok(())
}

fn sys_read(fd: usize, buf: &mut [u8]) -> io::Result<usize> {
    check(unsafe { syscall3(sysno::READ, fd, buf.as_mut_ptr() as usize, buf.len()) })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Esc,
    Unknown,
}

/// Holds the original terminal settings and restores them when dropped.
pub struct RawMode {
    orig: Termios,
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = sys_ioctl_set(STDIN_FD, &self.orig);
    }
}

/// Puts the terminal into raw mode (no line buffering, no echo).
/// Terminal settings are restored automatically when the returned
/// `RawMode` guard is dropped.
pub fn enable_raw_mode() -> io::Result<RawMode> {
    let orig = sys_ioctl_get(STDIN_FD)?;

    let mut raw = orig;
    raw.c_lflag &= !(ICANON | ECHO);
    raw.c_cc[VMIN] = 1;
    raw.c_cc[VTIME] = 0;
    sys_ioctl_set(STDIN_FD, &raw)?;

    Ok(RawMode { orig })
}

/// Clears the screen and moves the cursor to the top-left corner.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Clears the screen and reprints a constant header string,
/// keeping it pinned at the top on every redraw.
pub fn draw_header(text: &str) {
    clear_screen();
    println!("{text}");
    io::stdout().flush().unwrap();
}

/// Blocks until one key is available and returns it.
/// Requires raw mode to already be enabled via `enable_raw_mode`.
pub fn read_key() -> io::Result<Key> {
    let mut buf = [0u8; 1];
    if sys_read(STDIN_FD, &mut buf)? == 0 {
        return Ok(Key::Unknown);
    }

    match buf[0] {
        0x1b => {
            let mut seq = [0u8; 2];
            if sys_read(STDIN_FD, &mut seq[..1])? == 0 {
                return Ok(Key::Esc);
            }
            if sys_read(STDIN_FD, &mut seq[1..2])? == 0 {
                return Ok(Key::Esc);
            }
            if seq[0] == b'[' {
                match seq[1] {
                    b'A' => Ok(Key::Up),
                    b'B' => Ok(Key::Down),
                    b'C' => Ok(Key::Right),
                    b'D' => Ok(Key::Left),
                    _ => Ok(Key::Unknown),
                }
            } else {
                Ok(Key::Unknown)
            }
        }
        c => Ok(Key::Char(c as char)),
    }
}
