use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;
use x86_64::VirtAddr;

/// Process ID type
pub type Pid = u64;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Process priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// Process Control Block
#[derive(Debug)]
pub struct Process {
    pub pid: Pid,
    pub parent_pid: Option<Pid>,
    pub state: ProcessState,
    pub priority: Priority,
    pub stack_pointer: VirtAddr,
    pub instruction_pointer: VirtAddr,
    pub page_table: VirtAddr,
}

impl Process {
    /// Create a new process
    pub fn new(pid: Pid, parent_pid: Option<Pid>) -> Self {
        Self {
            pid,
            parent_pid,
            state: ProcessState::Ready,
            priority: Priority::Normal,
            stack_pointer: VirtAddr::new(0),
            instruction_pointer: VirtAddr::new(0),
            page_table: VirtAddr::new(0),
        }
    }

    /// Set process state
    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }

    /// Set process priority
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
}

/// Process Manager
pub struct ProcessManager {
    processes: Vec<Process>,
    ready_queue: VecDeque<Pid>,
    current_pid: Option<Pid>,
    next_pid: Pid,
}

impl ProcessManager {
    /// Create a new process manager
    pub const fn new() -> Self {
        Self {
            processes: Vec::new(),
            ready_queue: VecDeque::new(),
            current_pid: None,
            next_pid: 1,
        }
    }

    /// Create a new process
    pub fn create_process(&mut self, parent_pid: Option<Pid>) -> Pid {
        let pid = self.next_pid;
        self.next_pid += 1;

        let process = Process::new(pid, parent_pid);
        self.processes.push(process);
        self.ready_queue.push_back(pid);

        crate::serial_println!("[PM] Created process PID={}", pid);
        pid
    }

    /// Get process by PID
    pub fn get_process(&self, pid: Pid) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    /// Get mutable process by PID
    pub fn get_process_mut(&mut self, pid: Pid) -> Option<&mut Process> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }

    /// Schedule next process (simple round-robin)
    pub fn schedule(&mut self) -> Option<Pid> {
        // Move current process back to ready queue if still running
        if let Some(current) = self.current_pid {
            if let Some(process) = self.get_process(current) {
                if process.state == ProcessState::Running {
                    self.ready_queue.push_back(current);
                }
            }
        }

        // Get next ready process
        while let Some(pid) = self.ready_queue.pop_front() {
            if let Some(process) = self.get_process_mut(pid) {
                if process.state == ProcessState::Ready || process.state == ProcessState::Running {
                    process.state = ProcessState::Running;
                    self.current_pid = Some(pid);
                    return Some(pid);
                }
            }
        }

        None
    }

    /// Terminate a process
    pub fn terminate_process(&mut self, pid: Pid) {
        if let Some(process) = self.get_process_mut(pid) {
            process.state = ProcessState::Terminated;
            crate::serial_println!("[PM] Terminated process PID={}", pid);
        }

        if self.current_pid == Some(pid) {
            self.current_pid = None;
        }
    }

    /// Block a process
    pub fn block_process(&mut self, pid: Pid) {
        if let Some(process) = self.get_process_mut(pid) {
            process.state = ProcessState::Blocked;
        }
    }

    /// Unblock a process
    pub fn unblock_process(&mut self, pid: Pid) {
        if let Some(process) = self.get_process_mut(pid) {
            if process.state == ProcessState::Blocked {
                process.state = ProcessState::Ready;
                self.ready_queue.push_back(pid);
            }
        }
    }

    /// Get current running process PID
    pub fn current_pid(&self) -> Option<Pid> {
        self.current_pid
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get ready process count
    pub fn ready_count(&self) -> usize {
        self.ready_queue.len()
    }
}

/// Global process manager
pub static PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());

/// Initialize process management
pub fn init() {
    crate::serial_println!("[PM] Process manager initialized");
    
    // Create init process (PID 1)
    let mut pm = PROCESS_MANAGER.lock();
    let init_pid = pm.create_process(None);
    assert_eq!(init_pid, 1, "Init process must have PID 1");
}

/// Create a new process
pub fn create_process(parent_pid: Option<Pid>) -> Pid {
    PROCESS_MANAGER.lock().create_process(parent_pid)
}

/// Schedule next process
pub fn schedule() -> Option<Pid> {
    PROCESS_MANAGER.lock().schedule()
}

/// Terminate current process
pub fn exit(pid: Pid) {
    PROCESS_MANAGER.lock().terminate_process(pid);
}

/// Get current process PID
pub fn current_pid() -> Option<Pid> {
    PROCESS_MANAGER.lock().current_pid()
}
