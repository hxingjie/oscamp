//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{BaseAllocator, ByteAllocator, AllocResult, AllocError};
use core::ptr::NonNull;
use core::alloc::Layout;

extern crate buddy_system_allocator;
use buddy_system_allocator::Heap;

extern crate log;
use log::{debug, error, info, trace, warn};

pub struct LabByteAllocator {
    inner: Heap<32>,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            inner: Heap::<32>::new(),
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        unsafe { self.inner.init(start, size) };
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe { self.inner.add_to_heap(start, start + size) };
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.inner.alloc(layout).map_err(|_| AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.inner.dealloc(pos, layout)
    }

    fn total_bytes(&self) -> usize {
        self.inner.stats_total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.inner.stats_alloc_actual()
    }

    fn available_bytes(&self) -> usize {
        self.inner.stats_total_bytes() - self.inner.stats_alloc_actual()
    }
}
