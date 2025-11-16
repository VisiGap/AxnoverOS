/// Process management for FractureOS
/// This module handles process creation, scheduling, and lifecycle

use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessId(pub usize);

pub struct Process {
    pub pid: ProcessId,
    pub parent_pid: Option<ProcessId>,
    pub state: ProcessState,
    pub name: String,
    // TODO: Add page table, registers, stack pointer, etc.
}

impl Process {
    pub fn new(pid: ProcessId, name: String) -> Self {
        Process {
            pid,
            parent_pid: None,
            state: ProcessState::Ready,
            name,
        }
    }

    pub fn set_parent(&mut self, parent_pid: ProcessId) {
        self.parent_pid = Some(parent_pid);
    }
}

pub struct ProcessManager {
    processes: Vec<Process>,
    next_pid: usize,
    current_process: Option<ProcessId>,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: Vec::new(),
            next_pid: 1,
            current_process: None,
        }
    }

    pub fn create_process(&mut self, name: String) -> ProcessId {
        let pid = ProcessId(self.next_pid);
        self.next_pid += 1;

        let process = Process::new(pid, name);
        self.processes.push(process);

        pid
    }

    pub fn get_process(&self, pid: ProcessId) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn get_process_mut(&mut self, pid: ProcessId) -> Option<&mut Process> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }

    pub fn terminate_process(&mut self, pid: ProcessId) -> Result<(), &'static str> {
        if let Some(process) = self.get_process_mut(pid) {
            process.state = ProcessState::Terminated;
            Ok(())
        } else {
            Err("Process not found")
        }
    }

    pub fn current_process(&self) -> Option<ProcessId> {
        self.current_process
    }

    pub fn set_current_process(&mut self, pid: ProcessId) {
        self.current_process = Some(pid);
    }

    pub fn schedule_next(&mut self) -> Option<ProcessId> {
        // Simple round-robin scheduler
        // TODO: Implement more sophisticated scheduling
        self.processes
            .iter()
            .find(|p| p.state == ProcessState::Ready)
            .map(|p| p.pid)
    }
}

// Global process manager (will be properly initialized later)
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
}
