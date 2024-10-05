// src/min_heap.rs

pub struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    pub fn insert(&mut self, value: i32) {
        self.heap.push(value);
        self.bubble_up(self.heap.len() - 1);
    }

    fn bubble_up(&mut self, index: usize) {
        let mut child_index = index;

        while child_index > 0 {
            let parent_index = (child_index - 1) / 2;
            if self.heap[child_index] >= self.heap[parent_index] {
                break;
            }
            // Swap
            self.heap.swap(child_index, parent_index);
            child_index = parent_index;
        }
    }

    pub fn extract_min(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }

        let min_value = self.heap[0];
        let last_value = self.heap.pop().unwrap();

        if !self.heap.is_empty() {
            self.heap[0] = last_value;
            self.bubble_down(0);
        }

        Some(min_value)
    }

    fn bubble_down(&mut self, index: usize) {
        let mut smallest = index;
        let left_child = 2 * index + 1;
        let right_child = 2 * index + 2;

        if left_child < self.heap.len() && self.heap[left_child] < self.heap[smallest] {
            smallest = left_child;
        }

        if right_child < self.heap.len() && self.heap[right_child] < self.heap[smallest] {
            smallest = right_child;
        }

        if smallest != index {
            // Swap
            self.heap.swap(index, smallest);
            self.bubble_down(smallest);
        }
    }

    pub fn display(&self) {
        println!("Min Heap: {:?}", &self.heap);
    }
}
