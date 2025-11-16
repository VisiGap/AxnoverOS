use alloc::vec::Vec;
use spin::Mutex;
use x86_64::VirtAddr;
use crate::process::Pid;

/// Shared memory segment ID
pub type ShmId = u64;

/// Shared memory permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShmPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl ShmPermissions {
    pub const READ_ONLY: Self = Self {
        read: true,
        write: false,
        execute: false,
    };

    pub const READ_WRITE: Self = Self {
        read: true,
        write: true,
        execute: false,
    };

    pub const ALL: Self = Self {
        read: true,
        write: true,
        execute: true,
    };
}

/// Shared memory segment
#[derive(Debug)]
pub struct SharedMemory {
    id: ShmId,
    owner: Pid,
    size: usize,
    address: VirtAddr,
    attached_processes: Vec<(Pid, ShmPermissions)>,
}

impl SharedMemory {
    fn new(id: ShmId, owner: Pid, size: usize, address: VirtAddr) -> Self {
        Self {
            id,
            owner,
            size,
            address,
            attached_processes: Vec::new(),
        }
    }

    /// Attach a process to this shared memory
    fn attach(&mut self, pid: Pid, perms: ShmPermissions) -> Result<(), ShmError> {
        // Check if already attached
        if self.attached_processes.iter().any(|(p, _)| *p == pid) {
            return Err(ShmError::AlreadyAttached);
        }

        self.attached_processes.push((pid, perms));
        Ok(())
    }

    /// Detach a process from this shared memory
    fn detach(&mut self, pid: Pid) -> Result<(), ShmError> {
        let initial_len = self.attached_processes.len();
        self.attached_processes.retain(|(p, _)| *p != pid);

        if self.attached_processes.len() == initial_len {
            Err(ShmError::NotAttached)
        } else {
            Ok(())
        }
    }

    /// Check if a process has permission
    fn check_permission(&self, pid: Pid, write: bool) -> bool {
        for (p, perms) in &self.attached_processes {
            if *p == pid {
                return if write { perms.write } else { perms.read };
            }
        }
        false
    }

    /// Get number of attached processes
    fn attachment_count(&self) -> usize {
        self.attached_processes.len()
    }
}

/// Shared memory manager
pub struct ShmManager {
    segments: Vec<SharedMemory>,
    next_id: ShmId,
}

impl ShmManager {
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a new shared memory segment
    pub fn create(&mut self, owner: Pid, size: usize) -> Result<ShmId, ShmError> {
        if size == 0 || size > MAX_SHM_SIZE {
            return Err(ShmError::InvalidSize);
        }

        // Allocate virtual address (simplified - should use proper memory allocation)
        let address = VirtAddr::new(0x8000_0000_0000 + (self.next_id * 0x10000));

        let id = self.next_id;
        self.next_id += 1;

        let mut segment = SharedMemory::new(id, owner, size, address);
        segment.attach(owner, ShmPermissions::ALL)?;

        self.segments.push(segment);

        crate::serial_println!(
            "[SHM] Created segment {} for process {} ({} bytes)",
            id,
            owner,
            size
        );

        Ok(id)
    }

    /// Attach to a shared memory segment
    pub fn attach(&mut self, id: ShmId, pid: Pid, perms: ShmPermissions) -> Result<VirtAddr, ShmError> {
        for segment in &mut self.segments {
            if segment.id == id {
                segment.attach(pid, perms)?;
                crate::serial_println!(
                    "[SHM] Process {} attached to segment {}",
                    pid,
                    id
                );
                return Ok(segment.address);
            }
        }
        Err(ShmError::NotFound)
    }

    /// Detach from a shared memory segment
    pub fn detach(&mut self, id: ShmId, pid: Pid) -> Result<(), ShmError> {
        for segment in &mut self.segments {
            if segment.id == id {
                segment.detach(pid)?;
                crate::serial_println!(
                    "[SHM] Process {} detached from segment {}",
                    pid,
                    id
                );
                return Ok(());
            }
        }
        Err(ShmError::NotFound)
    }

    /// Delete a shared memory segment
    pub fn delete(&mut self, id: ShmId, pid: Pid) -> Result<(), ShmError> {
        for (i, segment) in self.segments.iter().enumerate() {
            if segment.id == id {
                // Only owner can delete
                if segment.owner != pid {
                    return Err(ShmError::PermissionDenied);
                }

                // Check if still attached
                if segment.attachment_count() > 0 {
                    return Err(ShmError::StillAttached);
                }

                self.segments.remove(i);
                crate::serial_println!("[SHM] Deleted segment {}", id);
                return Ok(());
            }
        }
        Err(ShmError::NotFound)
    }

    /// Get segment info
    pub fn get_info(&self, id: ShmId) -> Option<(usize, Pid, usize)> {
        for segment in &self.segments {
            if segment.id == id {
                return Some((segment.size, segment.owner, segment.attachment_count()));
            }
        }
        None
    }
}

/// Shared memory errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShmError {
    NotFound,
    InvalidSize,
    AlreadyAttached,
    NotAttached,
    PermissionDenied,
    StillAttached,
}

/// Maximum shared memory size (16MB)
pub const MAX_SHM_SIZE: usize = 16 * 1024 * 1024;

/// Global shared memory manager
pub static SHM_MANAGER: Mutex<ShmManager> = Mutex::new(ShmManager::new());

/// Initialize shared memory system
pub fn init() {
    crate::serial_println!("[SHM] Shared memory system initialized");
}

/// Create shared memory segment
pub fn create(owner: Pid, size: usize) -> Result<ShmId, ShmError> {
    SHM_MANAGER.lock().create(owner, size)
}

/// Attach to shared memory
pub fn attach(id: ShmId, pid: Pid, perms: ShmPermissions) -> Result<VirtAddr, ShmError> {
    SHM_MANAGER.lock().attach(id, pid, perms)
}

/// Detach from shared memory
pub fn detach(id: ShmId, pid: Pid) -> Result<(), ShmError> {
    SHM_MANAGER.lock().detach(id, pid)
}

/// Delete shared memory segment
pub fn delete(id: ShmId, pid: Pid) -> Result<(), ShmError> {
    SHM_MANAGER.lock().delete(id, pid)
}
