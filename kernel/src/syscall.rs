use crate::process::{self, Pid};
use x86_64::VirtAddr;
use x86_64::registers::model_specific::{LStar, SFMask, Star};
use x86_64::registers::rflags::RFlags;

/// System call numbers
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum SyscallNumber {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Fork = 57,
    Exec = 59,
    Exit = 60,
    Wait = 61,
    GetPid = 39,
    Mmap = 9,
    Munmap = 11,
}

impl SyscallNumber {
    pub fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Read),
            1 => Some(Self::Write),
            2 => Some(Self::Open),
            3 => Some(Self::Close),
            57 => Some(Self::Fork),
            59 => Some(Self::Exec),
            60 => Some(Self::Exit),
            61 => Some(Self::Wait),
            39 => Some(Self::GetPid),
            9 => Some(Self::Mmap),
            11 => Some(Self::Munmap),
            _ => None,
        }
    }
}

/// System call handler
pub fn syscall_handler(
    syscall_number: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
) -> u64 {
    let syscall = match SyscallNumber::from_u64(syscall_number) {
        Some(s) => s,
        None => {
            crate::serial_println!("[SYSCALL] Unknown syscall: {}", syscall_number);
            return u64::MAX; // Error
        }
    };

    match syscall {
        SyscallNumber::Write => sys_write(arg1 as i32, arg2 as *const u8, arg3 as usize),
        SyscallNumber::Read => sys_read(arg1 as i32, arg2 as *mut u8, arg3 as usize),
        SyscallNumber::Exit => sys_exit(arg1 as i32),
        SyscallNumber::GetPid => sys_getpid(),
        SyscallNumber::Fork => sys_fork(),
        SyscallNumber::Mmap => sys_send(arg1, arg2 as *const u8, arg3 as usize),
        SyscallNumber::Munmap => sys_receive(arg1 as *mut u8, arg2 as usize),
        _ => {
            crate::serial_println!("[SYSCALL] Unimplemented syscall: {:?}", syscall);
            u64::MAX
        }
    }
}

/// sys_write - Write to file descriptor
fn sys_write(fd: i32, buf: *const u8, count: usize) -> u64 {
    if fd != 1 && fd != 2 {
        return u64::MAX; // Only stdout/stderr supported
    }

    unsafe {
        let slice = core::slice::from_raw_parts(buf, count);
        for &byte in slice {
            crate::serial::SERIAL1.lock().send(byte);
        }
    }

    count as u64
}

/// sys_read - Read from file descriptor
fn sys_read(_fd: i32, _buf: *mut u8, _count: usize) -> u64 {
    // TODO: Implement read
    0
}

/// sys_exit - Terminate current process
fn sys_exit(status: i32) -> u64 {
    if let Some(pid) = process::current_pid() {
        crate::serial_println!("[SYSCALL] Process {} exiting with status {}", pid, status);
        process::exit(pid);
    }
    0
}

/// sys_getpid - Get current process ID
fn sys_getpid() -> u64 {
    process::current_pid().unwrap_or(0)
}

/// sys_fork - Create a new process
fn sys_fork() -> u64 {
    let parent_pid = process::current_pid();
    let child_pid = process::create_process(parent_pid);
    
    // Register child for IPC and signals
    crate::ipc::register_process(child_pid);
    crate::signal::register_process(child_pid);
    
    crate::serial_println!("[SYSCALL] Fork: parent={:?}, child={}", parent_pid, child_pid);
    
    child_pid
}

/// sys_send - Send IPC message
fn sys_send(receiver: u64, data_ptr: *const u8, data_len: usize) -> u64 {
    if let Some(sender) = process::current_pid() {
        unsafe {
            let data = core::slice::from_raw_parts(data_ptr, data_len);
            match crate::ipc::send_message(sender, receiver, data) {
                Ok(()) => 0,
                Err(_) => u64::MAX,
            }
        }
    } else {
        u64::MAX
    }
}

/// sys_receive - Receive IPC message
fn sys_receive(buffer_ptr: *mut u8, buffer_len: usize) -> u64 {
    if let Some(pid) = process::current_pid() {
        if let Some(msg) = crate::ipc::receive_message(pid) {
            let copy_len = core::cmp::min(msg.data.len(), buffer_len);
            unsafe {
                core::ptr::copy_nonoverlapping(
                    msg.data.as_ptr(),
                    buffer_ptr,
                    copy_len
                );
            }
            copy_len as u64
        } else {
            0
        }
    } else {
        u64::MAX
    }
}

/// sys_kill - Send signal to process
fn sys_kill(target: u64, signal: u32) -> u64 {
    if let Some(sig) = crate::signal::Signal::from_u32(signal) {
        let sender = process::current_pid();
        crate::signal::send_signal(target, sig, sender);
        0
    } else {
        u64::MAX
    }
}

/// Initialize system call handling
pub fn init() {
    use x86_64::registers::model_specific::{LStar, SFMask, Star};
    use x86_64::registers::rflags::RFlags;
    
    unsafe {
        // Set SYSCALL entry point
        LStar::write(VirtAddr::new(syscall_entry as u64));
        
        // Set segment selectors for SYSCALL/SYSRET
        // STAR[47:32] = Kernel CS (0x08)
        // STAR[63:48] = User CS (0x18)
        Star::write(0x0008, 0x0010, 0x0018, 0x0020).unwrap();
        
        // Set RFLAGS mask (clear IF and TF on syscall)
        SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG);
    }
    
    crate::serial_println!("[SYSCALL] System call handler initialized");
    crate::serial_println!("[SYSCALL] SYSCALL/SYSRET enabled");
}

/// Low-level syscall entry point
#[naked]
extern "C" fn syscall_entry() {
    unsafe {
        core::arch::asm!(
            // Save user stack
            "mov [gs:0x00], rsp",
            // Load kernel stack
            "mov rsp, [gs:0x08]",
            // Save registers
            "push rcx",  // User RIP
            "push r11",  // User RFLAGS
            "push rbp",
            "push rbx",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            // Call handler
            "mov rdi, rax",  // syscall number
            "mov rsi, rdi",  // arg1
            "mov rdx, rsi",  // arg2
            "mov rcx, rdx",  // arg3
            "call {}",
            // Restore registers
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbx",
            "pop rbp",
            "pop r11",
            "pop rcx",
            // Restore user stack
            "mov rsp, [gs:0x00]",
            // Return to userspace
            "sysretq",
            sym syscall_handler,
            options(noreturn)
        );
    }
}
