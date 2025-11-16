use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
        PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub const PHYSICAL_MEMORY_OFFSET: u64 = 0xFFFF_8000_0000_0000;

/// Initialize memory management
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Get active level 4 page table
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

/// Physical Memory Manager
pub struct PhysicalMemoryManager {
    memory_start: PhysAddr,
    memory_end: PhysAddr,
    next_free_frame: PhysAddr,
    bitmap: &'static mut [u64],
}

impl PhysicalMemoryManager {
    /// Create a new physical memory manager
    pub unsafe fn new(memory_start: PhysAddr, memory_end: PhysAddr, bitmap_addr: VirtAddr) -> Self {
        let total_frames = ((memory_end.as_u64() - memory_start.as_u64()) / 4096) as usize;
        let bitmap_size = (total_frames + 63) / 64;

        let bitmap = core::slice::from_raw_parts_mut(bitmap_addr.as_mut_ptr::<u64>(), bitmap_size);

        // Initialize bitmap - all frames free
        for entry in bitmap.iter_mut() {
            *entry = 0;
        }

        Self {
            memory_start,
            memory_end,
            next_free_frame: memory_start,
            bitmap,
        }
    }

    /// Allocate a physical frame
    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame_count = ((self.memory_end.as_u64() - self.memory_start.as_u64()) / 4096) as usize;

        for i in 0..frame_count {
            let bitmap_index = i / 64;
            let bit_index = i % 64;

            if self.bitmap[bitmap_index] & (1 << bit_index) == 0 {
                // Frame is free, mark as used
                self.bitmap[bitmap_index] |= 1 << bit_index;

                let frame_addr = self.memory_start + (i as u64 * 4096);
                return Some(PhysFrame::containing_address(frame_addr));
            }
        }

        None
    }

    /// Free a physical frame
    pub fn free_frame(&mut self, frame: PhysFrame) {
        let frame_addr = frame.start_address();
        let frame_index = ((frame_addr.as_u64() - self.memory_start.as_u64()) / 4096) as usize;

        let bitmap_index = frame_index / 64;
        let bit_index = frame_index % 64;

        self.bitmap[bitmap_index] &= !(1 << bit_index);
    }

    /// Get total memory in bytes
    pub fn total_memory(&self) -> u64 {
        self.memory_end.as_u64() - self.memory_start.as_u64()
    }

    /// Get used memory in bytes
    pub fn used_memory(&self) -> u64 {
        let mut used_frames = 0u64;
        for &entry in self.bitmap.iter() {
            used_frames += entry.count_ones() as u64;
        }
        used_frames * 4096
    }
}

/// Boot frame allocator
pub struct BootInfoFrameAllocator {
    next: usize,
    end: usize,
}

impl BootInfoFrameAllocator {
    /// Create a new frame allocator
    ///
    /// # Safety
    /// Caller must ensure memory range is valid
    pub unsafe fn init(start_addr: u64, end_addr: u64) -> Self {
        Self {
            next: (start_addr / 4096) as usize,
            end: (end_addr / 4096) as usize,
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        if self.next < self.end {
            let frame = PhysFrame::containing_address(PhysAddr::new((self.next as u64) * 4096));
            self.next += 1;
            Some(frame)
        } else {
            None
        }
    }
}

/// Map a page to a frame
pub fn map_page(
    page: Page,
    frame: PhysFrame,
    flags: PageTableFlags,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }
    Ok(())
}
