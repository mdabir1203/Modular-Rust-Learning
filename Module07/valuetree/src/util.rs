// src/utils.rs
/*
optimize_memory_usage: Converts a Vec to a VecDeque for more efficient insertions and deletions if those operations are frequent.
reuse_buffers: Resizes existing buffers instead of creating new ones, reducing allocation overhead.
handle_large_data: Processes large datasets in chunks to avoid high memory usage from large allocations.

*/
use std::collections::VecDeque;

// Optimizes memory usage by using a memory-efficient data structure
pub fn optimize_memory_usage<T>(data: Vec<T>) -> VecDeque<T> {
    let mut optimized_data = VecDeque::with_capacity(data.len());
    for item in data {
        optimized_data.push_back(item);
    }
    optimized_data
}

// Function to avoid excessive allocations by reusing existing buffers
pub fn reuse_buffers<T>(existing: &mut Vec<T>, new_capacity: usize) {
    if existing.capacity() < new_capacity {
        existing.reserve(new_capacity - existing.len());
    }
}

// Efficiently handle large data by using slices
pub fn handle_large_data<T>(data: &[T]) {
    // Example operation: Process slices of data to avoid large allocations
    for _chunk in data.chunks(1024) {
        // Process each chunk of data
    }
}
