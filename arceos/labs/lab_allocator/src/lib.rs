//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{BaseAllocator, ByteAllocator, AllocResult, AllocError};
use core::ptr::NonNull;
use core::alloc::Layout;

extern crate log;
use log::{debug, error, info, trace, warn};

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

pub struct LabByteAllocator {
    total_bytes: usize,
    used_bytes: usize,

    mem: [MemUnit; 8192],
    mem_sz: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        LabByteAllocator {
            total_bytes: 0,
            used_bytes: 0,

            mem: [MemUnit::new(); 8192],
            mem_sz: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        //unimplemented!();
        warn!("Init EarlyAllocator, {}, {}", start, size);

        self.mem[self.mem_sz].ptr_byte = start;
        self.mem[self.mem_sz].size = size;

        self.mem_sz += 1;

        self.total_bytes = size;
        
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        warn!("Add Memory, {}, {}", start, size);
        for i in 0..self.mem_sz {
            if (start >= self.mem[i].ptr_byte && start <= self.mem[i].ptr_byte + self.mem[i].size-1)
                || (start+size-1 >= self.mem[i].ptr_byte && start+size-1 <= self.mem[i].ptr_byte + self.mem[i].size-1) {
                return Err(AllocError::MemoryOverlap);
            }
        }

        self.mem[self.mem_sz].ptr_byte = start;
        self.mem[self.mem_sz].size = size;

        self.mem_sz += 1;

        self.total_bytes += size;

        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();

        warn!("alloc: {}, align: {}", size, align);

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
                    warn!("total_bytes: {}, used_bytes: {}, available_bytes: {}", self.total_bytes(), self.used_bytes, self.available_bytes());
                    return Ok(NonNull::new_unchecked(front_addr as *mut u8));
                }
            }
        }
        warn!("total_bytes: {}, used_bytes: {}, available_bytes: {}", self.total_bytes(), self.used_bytes(), self.available_bytes());
        Err(AllocError::NoMemory)
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        //unimplemented!();
    }
    fn total_bytes(&self) -> usize {
        //unimplemented!();
        self.total_bytes
    }
    fn used_bytes(&self) -> usize {
        //unimplemented!();
        self.used_bytes
    }
    fn available_bytes(&self) -> usize {
        //unimplemented!();
        self.total_bytes - self.used_bytes
    }
}
