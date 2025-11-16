/// System call interface for FractureOS
/// This module will handle user-space to kernel-space transitions

use crate::println;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Syscall {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Exit = 60,
    Fork = 57,
    Execve = 59,
    GetPid = 39,
    // Add more syscalls as needed
}

impl Syscall {
    pub fn from_id(id: usize) -> Option<Self> {
        match id {
            0 => Some(Syscall::Read),
            1 => Some(Syscall::Write),
            2 => Some(Syscall::Open),
            3 => Some(Syscall::Close),
            39 => Some(Syscall::GetPid),
            57 => Some(Syscall::Fork),
            59 => Some(Syscall::Execve),
            60 => Some(Syscall::Exit),
            _ => None,
        }
    }
}

/// Handle a system call
pub fn handle_syscall(
    syscall_id: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> isize {
    let syscall = match Syscall::from_id(syscall_id) {
        Some(sc) => sc,
        None => {
            println!("[SYSCALL] Unknown syscall: {}", syscall_id);
            return -1;
        }
    };

    match syscall {
        Syscall::Write => sys_write(arg1, arg2, arg3),
        Syscall::Read => sys_read(arg1, arg2, arg3),
        Syscall::Exit => sys_exit(arg1),
        Syscall::GetPid => sys_getpid(),
        _ => {
            println!("[SYSCALL] Unimplemented syscall: {:?}", syscall);
            -1
        }
    }
}

fn sys_write(fd: usize, buf: usize, count: usize) -> isize {
    // TODO: Implement proper file descriptor handling
    println!("[SYSCALL] write(fd={}, buf={:#x}, count={})", fd, buf, count);
    count as isize
}

fn sys_read(fd: usize, buf: usize, count: usize) -> isize {
    // TODO: Implement proper file descriptor handling
    println!("[SYSCALL] read(fd={}, buf={:#x}, count={})", fd, buf, count);
    0
}

fn sys_exit(code: usize) -> isize {
    println!("[SYSCALL] exit(code={})", code);
    // TODO: Implement process termination
    0
}

fn sys_getpid() -> isize {
    // TODO: Return actual process ID
    println!("[SYSCALL] getpid()");
    1 // Return dummy PID for now
}
