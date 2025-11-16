use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, null_mut};
use spin::Mutex;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB

/// A simple linked list allocator
pub struct LinkedListAllocator {
    head: Mutex<Option<&'static mut Node>>,
}

struct Node {
    size: usize,
    next: Option<&'static mut Node>,
}

impl Node {
    const fn new(size: usize) -> Self {
        Node { size, next: None }
    }
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: Mutex::new(None),
        }
    }

    /// Initialize the allocator with the given heap bounds
    /// 
    /// # Safety
    /// This function must be called only once and the heap bounds must be valid
    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        let mut head = self.head.lock();
        *head = Some(&mut *(heap_start as *mut Node));
        if let Some(node) = head.as_mut() {
            node.size = heap_size;
            node.next = None;
        }
    }

    fn alloc_from_region(region: &Node, layout: Layout) -> Option<(*mut u8, &'static mut Node)> {
        let alloc_start = align_up(region as *const Node as usize + core::mem::size_of::<Node>(), layout.align());
        let alloc_end = alloc_start.checked_add(layout.size())?;

        let region_start = region as *const Node as usize;
        let region_end = region_start.checked_add(region.size)?;

        if alloc_end > region_end {
            return None;
        }

        let excess_size = region_end - alloc_end;
        if excess_size > 0 && excess_size < core::mem::size_of::<Node>() {
            return None;
        }

        Some((alloc_start as *mut u8, unsafe {
            &mut *(region_start as *mut Node)
        }))
    }
}

unsafe impl GlobalAlloc for LinkedListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut head = self.head.lock();
        let mut current = head.as_deref_mut();
        let mut prev: Option<&mut Node> = None;

        while let Some(region) = current {
            if let Some((alloc_start, _)) = Self::alloc_from_region(region, layout) {
                let alloc_end = alloc_start as usize + layout.size();
                let region_ptr = region as *mut Node;
                let region_start = region_ptr as usize;
                let region_end = region_start + region.size;

                if alloc_end < region_end {
                    let new_node = &mut *((alloc_end) as *mut Node);
                    new_node.size = region_end - alloc_end;
                    new_node.next = region.next.take();

                    if let Some(prev) = prev {
                        prev.next = Some(new_node);
                    } else {
                        *head = Some(new_node);
                    }
                } else {
                    if let Some(prev) = prev {
                        prev.next = region.next.take();
                    } else {
                        *head = region.next.take();
                    }
                }

                return alloc_start;
            }

            prev = Some(region);
            current = region.next.as_deref_mut();
        }

        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let new_node = &mut *(ptr as *mut Node);
        new_node.size = layout.size();
        new_node.next = None;

        let mut head = self.head.lock();
        
        if head.is_none() {
            *head = Some(new_node);
            return;
        }

        let mut current = head.as_mut();
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                return;
            }
            current = node.next.as_mut();
        }
    }
}

#[global_allocator]
static ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

/// Initialize the heap
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe {
        ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
