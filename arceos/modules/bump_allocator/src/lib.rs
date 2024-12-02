// EarlyAllocator

// make run A=exercises/alt_alloc/ 
#![no_std]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

use core::alloc::Layout;
use core::ptr::NonNull;

extern crate log;
use log::{debug, error, info, trace, warn};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///

#[derive(Clone, Copy)]
struct MemUnit {
    ptr_byte: usize,
    ptr_page: usize,
    size: usize,
}
impl MemUnit {
    const fn new() -> Self {
        MemUnit { ptr_byte: 0, ptr_page: 0, size: 0 }
    }
}
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    total_bytes: usize,
    used_bytes: usize,

    total_pages: usize,
    used_pages: usize,

    mem: [MemUnit; 8192],
    mem_sz: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        EarlyAllocator {
            total_bytes: 0,
            used_bytes: 0,

            total_pages: 0,
            used_pages: 0,

            mem: [MemUnit::new(); 8192],
            mem_sz: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE>{
    /// Initialize the allocator with a free memory region.
    fn init(&mut self, start: usize, size: usize) {
        warn!("Init EarlyAllocator");

        self.mem[self.mem_sz].ptr_byte = start;
        self.mem[self.mem_sz].ptr_page = start+size-PAGE_SIZE;
        self.mem[self.mem_sz].size = size;

        self.mem_sz += 1;

        self.total_bytes = size;
        self.total_pages = size / PAGE_SIZE;
    }

    /// Add a free memory region to the allocator.
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        // 检查获得的内存是否有重叠

        for i in 0..self.mem_sz {
            if (start >= self.mem[i].ptr_byte && start <= self.mem[i].ptr_byte + self.mem[i].size-1)
                || (start+size-1 >= self.mem[i].ptr_byte && start+size-1 <= self.mem[i].ptr_byte + self.mem[i].size-1) {
                return Err(AllocError::MemoryOverlap);
            }
        }

        self.mem[self.mem_sz].ptr_byte = start;
        self.mem[self.mem_sz].ptr_page = start+size-PAGE_SIZE;
        self.mem[self.mem_sz].size = size;

        self.mem_sz += 1;

        self.total_bytes += size;
        self.total_pages += size / PAGE_SIZE;

        Ok(())
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();

        for i in 0..self.mem_sz {
            let mut front_addr = self.mem[i].ptr_byte;
            while front_addr % align != 0 { // align
                front_addr += 1;
            }
            let back_addr = front_addr+size-1;
            if back_addr < self.mem[i].ptr_page {
                self.mem[i].ptr_byte += size;
                self.used_bytes += size;
                unsafe {
                    return Ok(NonNull::new_unchecked(front_addr as *mut u8));
                }
            }
        }

        Err(AllocError::NoMemory)
    }

    /// Deallocate memory at the given position, size, and alignment.
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {

    }

    /// Returns total memory size in bytes.
    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    /// Returns allocated memory size in bytes.
    fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    /// Returns available memory size in bytes.
    fn available_bytes(&self) -> usize {
        self.total_bytes - self.used_bytes
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    /// The size of a memory page.
    const PAGE_SIZE: usize = PAGE_SIZE;

    /// Allocate contiguous memory pages with given count and alignment.
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align = 1_usize << align_pow2;

        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }

        for i in 0..self.mem_sz {
            let mut front_addr = self.mem[i].ptr_page;
            while front_addr % align != 0 {
                front_addr -= 1;
            }
            let mut page_sz = 0;
            while front_addr > self.mem[i].ptr_byte { // align
                page_sz += 1;
                if page_sz == num_pages {
                    self.mem[i].ptr_page = front_addr - PAGE_SIZE;
                    self.used_pages += num_pages;
                    return Ok(front_addr);
                }
                front_addr -= PAGE_SIZE;
            }
        }

        Err(AllocError::NoMemory)
    }

    /// Deallocate contiguous memory pages with given position and count.
    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {

    }

    /// Returns the total number of memory pages.
    fn total_pages(&self) -> usize {
        self.total_pages
    }

    /// Returns the number of allocated memory pages.
    fn used_pages(&self) -> usize {
        self.used_pages
    }

    /// Returns the number of available memory pages.
    fn available_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
}