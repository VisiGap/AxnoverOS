use crate::process::Pid;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

/// Maximum message size (4KB)
pub const MAX_MESSAGE_SIZE: usize = 4096;

/// IPC Message
#[derive(Debug, Clone)]
pub struct Message {
    pub sender: Pid,
    pub receiver: Pid,
    pub data: Vec<u8>,
    pub msg_type: MessageType,
}

/// Message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Data,
    Signal,
    Request,
    Response,
}

impl Message {
    pub fn new(sender: Pid, receiver: Pid, data: Vec<u8>, msg_type: MessageType) -> Self {
        Self {
            sender,
            receiver,
            data,
            msg_type,
        }
    }
}

/// Message queue for a process
#[derive(Debug)]
struct MessageQueue {
    messages: VecDeque<Message>,
    max_size: usize,
}

impl MessageQueue {
    fn new(max_size: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_size,
        }
    }

    fn push(&mut self, msg: Message) -> Result<(), IpcError> {
        if self.messages.len() >= self.max_size {
            return Err(IpcError::QueueFull);
        }
        self.messages.push_back(msg);
        Ok(())
    }

    fn pop(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    fn len(&self) -> usize {
        self.messages.len()
    }
}

/// IPC Manager
pub struct IpcManager {
    queues: Vec<(Pid, MessageQueue)>,
}

impl IpcManager {
    pub const fn new() -> Self {
        Self { queues: Vec::new() }
    }

    /// Register a process for IPC
    pub fn register_process(&mut self, pid: Pid) {
        self.queues.push((pid, MessageQueue::new(64)));
        crate::serial_println!("[IPC] Registered process {}", pid);
    }

    /// Unregister a process
    pub fn unregister_process(&mut self, pid: Pid) {
        self.queues.retain(|(p, _)| *p != pid);
        crate::serial_println!("[IPC] Unregistered process {}", pid);
    }

    /// Send a message
    pub fn send(&mut self, msg: Message) -> Result<(), IpcError> {
        // Find receiver's queue
        for (pid, queue) in &mut self.queues {
            if *pid == msg.receiver {
                queue.push(msg.clone())?;
                crate::serial_println!(
                    "[IPC] Message sent: {} -> {} ({} bytes)",
                    msg.sender,
                    msg.receiver,
                    msg.data.len()
                );
                return Ok(());
            }
        }
        Err(IpcError::ProcessNotFound)
    }

    /// Receive a message (blocking)
    pub fn receive(&mut self, pid: Pid) -> Option<Message> {
        for (p, queue) in &mut self.queues {
            if *p == pid {
                return queue.pop();
            }
        }
        None
    }

    /// Check if messages are available
    pub fn has_messages(&self, pid: Pid) -> bool {
        for (p, queue) in &self.queues {
            if *p == pid {
                return queue.len() > 0;
            }
        }
        false
    }

    /// Get message count for a process
    pub fn message_count(&self, pid: Pid) -> usize {
        for (p, queue) in &self.queues {
            if *p == pid {
                return queue.len();
            }
        }
        0
    }
}

/// IPC Error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    QueueFull,
    ProcessNotFound,
    MessageTooLarge,
    InvalidMessage,
}

/// Global IPC manager
pub static IPC_MANAGER: Mutex<IpcManager> = Mutex::new(IpcManager::new());

/// Initialize IPC system
pub fn init() {
    crate::serial_println!("[IPC] Message-passing IPC initialized");
}

/// Send a message to another process
pub fn send_message(sender: Pid, receiver: Pid, data: &[u8]) -> Result<(), IpcError> {
    if data.len() > MAX_MESSAGE_SIZE {
        return Err(IpcError::MessageTooLarge);
    }

    let msg = Message::new(sender, receiver, data.to_vec(), MessageType::Data);

    IPC_MANAGER.lock().send(msg)
}

/// Receive a message
pub fn receive_message(pid: Pid) -> Option<Message> {
    IPC_MANAGER.lock().receive(pid)
}

/// Check if messages are available
pub fn has_messages(pid: Pid) -> bool {
    IPC_MANAGER.lock().has_messages(pid)
}

/// Register process for IPC
pub fn register_process(pid: Pid) {
    IPC_MANAGER.lock().register_process(pid);
}

/// Unregister process from IPC
pub fn unregister_process(pid: Pid) {
    IPC_MANAGER.lock().unregister_process(pid);
}
