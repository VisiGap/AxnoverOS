use alloc::vec::Vec;
use spin::Mutex;
use crate::process::Pid;

/// Signal numbers (POSIX-like)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Signal {
    SIGHUP = 1,     // Hangup
    SIGINT = 2,     // Interrupt
    SIGQUIT = 3,    // Quit
    SIGILL = 4,     // Illegal instruction
    SIGTRAP = 5,    // Trace trap
    SIGABRT = 6,    // Abort
    SIGBUS = 7,     // Bus error
    SIGFPE = 8,     // Floating point exception
    SIGKILL = 9,    // Kill (cannot be caught)
    SIGUSR1 = 10,   // User-defined signal 1
    SIGSEGV = 11,   // Segmentation fault
    SIGUSR2 = 12,   // User-defined signal 2
    SIGPIPE = 13,   // Broken pipe
    SIGALRM = 14,   // Alarm clock
    SIGTERM = 15,   // Termination
    SIGCHLD = 17,   // Child status changed
    SIGCONT = 18,   // Continue
    SIGSTOP = 19,   // Stop (cannot be caught)
    SIGTSTP = 20,   // Terminal stop
}

impl Signal {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            1 => Some(Self::SIGHUP),
            2 => Some(Self::SIGINT),
            3 => Some(Self::SIGQUIT),
            4 => Some(Self::SIGILL),
            5 => Some(Self::SIGTRAP),
            6 => Some(Self::SIGABRT),
            7 => Some(Self::SIGBUS),
            8 => Some(Self::SIGFPE),
            9 => Some(Self::SIGKILL),
            10 => Some(Self::SIGUSR1),
            11 => Some(Self::SIGSEGV),
            12 => Some(Self::SIGUSR2),
            13 => Some(Self::SIGPIPE),
            14 => Some(Self::SIGALRM),
            15 => Some(Self::SIGTERM),
            17 => Some(Self::SIGCHLD),
            18 => Some(Self::SIGCONT),
            19 => Some(Self::SIGSTOP),
            20 => Some(Self::SIGTSTP),
            _ => None,
        }
    }

    pub fn is_catchable(&self) -> bool {
        !matches!(self, Signal::SIGKILL | Signal::SIGSTOP)
    }
}

/// Signal action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalAction {
    Default,
    Ignore,
    Handler(u64), // Handler function address
}

/// Signal handler entry
#[derive(Debug, Clone)]
struct SignalHandler {
    signal: Signal,
    action: SignalAction,
}

/// Pending signal
#[derive(Debug, Clone)]
struct PendingSignal {
    signal: Signal,
    sender: Option<Pid>,
}

/// Signal manager for a process
#[derive(Debug)]
pub struct ProcessSignals {
    pid: Pid,
    handlers: Vec<SignalHandler>,
    pending: Vec<PendingSignal>,
    blocked: Vec<Signal>,
}

impl ProcessSignals {
    pub fn new(pid: Pid) -> Self {
        Self {
            pid,
            handlers: Vec::new(),
            pending: Vec::new(),
            blocked: Vec::new(),
        }
    }

    /// Set signal handler
    pub fn set_handler(&mut self, signal: Signal, action: SignalAction) {
        // Remove existing handler
        self.handlers.retain(|h| h.signal != signal);
        
        // Add new handler
        self.handlers.push(SignalHandler { signal, action });
    }

    /// Get signal handler
    pub fn get_handler(&self, signal: Signal) -> SignalAction {
        for handler in &self.handlers {
            if handler.signal == signal {
                return handler.action;
            }
        }
        SignalAction::Default
    }

    /// Add pending signal
    pub fn add_pending(&mut self, signal: Signal, sender: Option<Pid>) {
        if !self.is_blocked(signal) {
            self.pending.push(PendingSignal { signal, sender });
        }
    }

    /// Get next pending signal
    pub fn next_pending(&mut self) -> Option<(Signal, Option<Pid>)> {
        if let Some(pending) = self.pending.pop() {
            Some((pending.signal, pending.sender))
        } else {
            None
        }
    }

    /// Block a signal
    pub fn block(&mut self, signal: Signal) {
        if !self.blocked.contains(&signal) {
            self.blocked.push(signal);
        }
    }

    /// Unblock a signal
    pub fn unblock(&mut self, signal: Signal) {
        self.blocked.retain(|s| *s != signal);
    }

    /// Check if signal is blocked
    pub fn is_blocked(&self, signal: Signal) -> bool {
        self.blocked.contains(&signal)
    }

    /// Has pending signals
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }
}

/// Global signal manager
pub struct SignalManager {
    process_signals: Vec<ProcessSignals>,
}

impl SignalManager {
    pub const fn new() -> Self {
        Self {
            process_signals: Vec::new(),
        }
    }

    /// Register process for signals
    pub fn register_process(&mut self, pid: Pid) {
        self.process_signals.push(ProcessSignals::new(pid));
    }

    /// Unregister process
    pub fn unregister_process(&mut self, pid: Pid) {
        self.process_signals.retain(|ps| ps.pid != pid);
    }

    /// Send signal to process
    pub fn send_signal(&mut self, target: Pid, signal: Signal, sender: Option<Pid>) {
        for ps in &mut self.process_signals {
            if ps.pid == target {
                ps.add_pending(signal, sender);
                crate::serial_println!(
                    "[SIGNAL] Sent {:?} to process {} from {:?}",
                    signal,
                    target,
                    sender
                );
                return;
            }
        }
    }

    /// Set signal handler
    pub fn set_handler(&mut self, pid: Pid, signal: Signal, action: SignalAction) {
        for ps in &mut self.process_signals {
            if ps.pid == pid {
                ps.set_handler(signal, action);
                return;
            }
        }
    }

    /// Process pending signals for a process
    pub fn process_signals(&mut self, pid: Pid) -> Option<(Signal, SignalAction)> {
        for ps in &mut self.process_signals {
            if ps.pid == pid {
                if let Some((signal, _sender)) = ps.next_pending() {
                    let action = ps.get_handler(signal);
                    return Some((signal, action));
                }
            }
        }
        None
    }
}

pub static SIGNAL_MANAGER: Mutex<SignalManager> = Mutex::new(SignalManager::new());

/// Initialize signal system
pub fn init() {
    crate::serial_println!("[SIGNAL] Signal system initialized");
}

/// Send signal to process
pub fn send_signal(target: Pid, signal: Signal, sender: Option<Pid>) {
    SIGNAL_MANAGER.lock().send_signal(target, signal, sender);
}

/// Set signal handler
pub fn set_handler(pid: Pid, signal: Signal, action: SignalAction) {
    SIGNAL_MANAGER.lock().set_handler(pid, signal, action);
}

/// Register process for signals
pub fn register_process(pid: Pid) {
    SIGNAL_MANAGER.lock().register_process(pid);
}

/// Unregister process
pub fn unregister_process(pid: Pid) {
    SIGNAL_MANAGER.lock().unregister_process(pid);
}

/// Process pending signals
pub fn process_signals(pid: Pid) -> Option<(Signal, SignalAction)> {
    SIGNAL_MANAGER.lock().process_signals(pid)
}
