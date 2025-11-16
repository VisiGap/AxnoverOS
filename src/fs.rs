/// Virtual File System for FractureOS
/// Provides a unified interface for different file systems

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    CharDevice,
    BlockDevice,
    Symlink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        FilePermissions { read, write, execute }
    }

    pub fn read_only() -> Self {
        FilePermissions::new(true, false, false)
    }

    pub fn read_write() -> Self {
        FilePermissions::new(true, true, false)
    }
}

pub struct Inode {
    pub id: usize,
    pub file_type: FileType,
    pub permissions: FilePermissions,
    pub size: usize,
    pub name: String,
}

impl Inode {
    pub fn new(id: usize, name: String, file_type: FileType) -> Self {
        Inode {
            id,
            file_type,
            permissions: FilePermissions::read_write(),
            size: 0,
            name,
        }
    }
}

pub trait FileSystem {
    fn read(&self, inode: &Inode, offset: usize, buf: &mut [u8]) -> Result<usize, &'static str>;
    fn write(&mut self, inode: &Inode, offset: usize, buf: &[u8]) -> Result<usize, &'static str>;
    fn create(&mut self, parent: &Inode, name: String, file_type: FileType) -> Result<Inode, &'static str>;
    fn delete(&mut self, inode: &Inode) -> Result<(), &'static str>;
    fn list(&self, dir: &Inode) -> Result<Vec<Inode>, &'static str>;
}

pub struct VirtualFileSystem {
    root: Inode,
    next_inode_id: usize,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        let root = Inode::new(0, String::from("/"), FileType::Directory);
        VirtualFileSystem {
            root,
            next_inode_id: 1,
        }
    }

    pub fn root(&self) -> &Inode {
        &self.root
    }

    fn allocate_inode_id(&mut self) -> usize {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        id
    }
}

// Global VFS instance
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VFS: Mutex<VirtualFileSystem> = Mutex::new(VirtualFileSystem::new());
}
