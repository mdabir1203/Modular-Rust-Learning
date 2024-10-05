// src/dynamic_array.rs

pub struct DynamicArray {
    size: usize,
    capacity: usize,
    array: Vec<i32>,
}

impl DynamicArray {
    pub fn new() -> Self {
        DynamicArray {
            size: 0,
            capacity: 1,
            array: vec![0; 1],
        }
    }

    pub fn append(&mut self, value: i32) {
        if self.size == self.capacity {
            self.resize(2 * self.capacity);
        }
        self.array[self.size] = value;
        self.size += 1;
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_array = vec![0; new_capacity];
        for i in 0..self.size {
            new_array[i] = self.array[i];
        }
        self.array = new_array;
        self.capacity = new_capacity;
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index < self.size {
            Some(self.array[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn display(&self) {
        println!("Dynamic Array: {:?}", &self.array[..self.size]);
    }
}
