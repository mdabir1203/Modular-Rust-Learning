use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]  // Deriving Clone for Item
struct Item<K, V> {
    key: K,
    value: V,
}

#[derive(Debug, Clone)]  // Deriving Clone for HashMap
struct HashMap<K, V> {
    buckets: Vec<Option<Item<K, V>>>,
    len: usize,
    initial_block_size: usize,
    capacity_factor: f64,
}

impl<K: Hash + Eq + Copy + Default + Clone, V: Clone + Default> HashMap<K, V> {
    // Constructor for initializing the HashMap
    fn new(initial_block_size: usize, capacity_factor: f64) -> Self {
        assert!(capacity_factor > 0.0 && capacity_factor <= 1.0, "Capacity factor must be between 0 and 1.");
        HashMap {
            buckets: vec![None; initial_block_size],
            len: 0,
            initial_block_size,
            capacity_factor,
        }
    }

    // Compute the index for the given key
    fn get_bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    // Add or update an item in the HashMap
    fn add_item(&mut self, key: K, value: V) {
        if self.is_full() {
            self.resize(self.buckets.len() * 2);
        }

        let mut index = self.get_bucket_index(&key);
        
        // Handle collision using linear probing
        loop {
            match &self.buckets[index] {
                Some(item) if item.key == key => {
                    // Update existing item
                    self.buckets[index] = Some(Item { key, value });
                    return;
                }
                None => {
                    // Insert new item
                    self.buckets[index] = Some(Item { key, value });
                    self.len += 1;
                    return;
                }
                _ => {
                    // Move to the next bucket in case of collision
                    index = (index + 1) % self.buckets.len();
                }
            }
        }
    }

    // Check if the current load exceeds the capacity factor
    fn is_full(&self) -> bool {
        (self.len as f64) >= (self.buckets.len() as f64 * self.capacity_factor)
    }

    // Resize the internal bucket array when capacity is reached
    fn resize(&mut self, new_size: usize) {
        let old_buckets = std::mem::take(&mut self.buckets);  // Move old buckets out
        self.buckets = vec![None; new_size];
        self.len = 0;

        // Rehash and insert items into the new resized buckets
        for item in old_buckets.into_iter().flatten() {
            self.add_item(item.key, item.value);
        }
    }

    // Retrieve a reference to the value corresponding to the key
    fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.get_bucket_index(key);
        
        // Search for the key using linear probing
        for _ in 0..self.buckets.len() {
            match &self.buckets[index] {
                Some(item) if item.key == *key => return Some(&item.value),
                None => return None,
                _ => {
                    index = (index + 1) % self.buckets.len();
                }
            }
        }

        None
    }

    // Remove the item corresponding to the key, returning the removed value if present
    fn remove(&mut self, key: &K) -> Option<V> {
        let mut index = self.get_bucket_index(key);

        // Search for the item using linear probing
        for _ in 0..self.buckets.len() {
            match &mut self.buckets[index] {
                Some(item) if item.key == *key => {
                    // Take the item out and decrement the size
                    let removed_item = self.buckets[index].take();
                    self.len -= 1;
                    return removed_item.map(|item| item.value);
                },
                None => return None,
                _ => {
                    index = (index + 1) % self.buckets.len();
                }
            }
        }

        None
    }

    // Return the current number of items in the map
    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
   let mut map = HashMap::<i32, i32>::new(4, 0.85);

   // Insert key-value pairs
   map.add_item(10, 20);
   map.add_item(25, 30);
   map.add_item(49, 40);
   
   // Retrieve values and check length
   println!("{:?}", map.get(&10));  // Output: Some(20)
   println!("{:?}", map.get(&49));  // Output: Some(20)
   
   println!("Length of map is {}", map.len());  // Output: Length of map is 2

   // Remove a value
   println!("Removing key 25: {:?}", map.remove(&25));  // Output: Some(30)
   println!("Length of map after removal: {}", map.len());  // Output: 1
}
