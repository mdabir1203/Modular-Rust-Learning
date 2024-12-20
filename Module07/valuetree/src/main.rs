use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::Arc;

pub struct DataStorage<K, V> {
    store: BTreeMap<K, V>,
}

impl<K: Ord, V> DataStorage<K, V> {
    pub fn new() -> Self {
        DataStorage {
            store: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key)
    }
}



fn main() {
    // Create a new DataStorage instance for storing i32 keys and String values
    let mut storage = DataStorage::<i32, String>::new();

    // Insert some data
    storage.insert(1, "One".to_string());
    storage.insert(2, "Two".to_string());
    storage.insert(3, "Three".to_string());

    // Retrieve and print data
    if let Some(value) = storage.get(&2) {
        println!("Value for key 2: {}", value);
    }

    // Try to get a non-existent key
    if let Some(value) = storage.get(&4) {
        println!("Value for key 4: {}", value);
    } else {
        println!("No value found for key 4");
    }
}

// Help me fix the unitttest to do benchmark 
// Help me fix the unitttest to do benchmark 
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_insertion() {
        let mut storage = DataStorage::new();
        storage.insert(10, "ten".to_string());
        assert_eq!(storage.get(&10), Some(&"ten".to_string()));
    }
    
    #[test]
    fn test_deletion() {
        let mut storage = DataStorage::new();
        storage.insert(20, "twenty".to_string());
        
        let removed_value = storage.remove(&20);
        assert_eq!(removed_value, Some("twenty".to_string()));
        assert_eq!(storage.get(&20), None);
    }

    #[test]
    fn test_deletion_nonexistent_key() {
        let mut storage = DataStorage::new();
        storage.insert(30, "thirty".to_string());
        
        let removed_value = storage.remove(&40); // Attempt to remove a non-existent key
        assert_eq!(removed_value, None); // Should return None
    }

    #[test]
    fn test_insertion_overwrite() {
        let mut storage = DataStorage::new();
        storage.insert(50, "fifty".to_string());
        storage.insert(50, "fifty updated".to_string()); // Overwrite existing key
        
        assert_eq!(storage.get(&50), Some(&"fifty updated".to_string()));
    }

    #[test]
    fn test_empty_storage() {
        let mut storage: DataStorage<i32, String> = DataStorage::new();
        assert_eq!(storage.get(&1), None); // Should return None for any key
        assert_eq!(storage.remove(&1), None); // Should return None when removing a non-existent key
    }

    #[test]
    fn test_performance() {
        let mut storage = DataStorage::new();
        let start = Instant::now();
        
        for i in 0..10000 {
            storage.insert(i, i.to_string());
        }
        
        let duration = start.elapsed();
        println!("Insertion took {:?}", duration);
        
        assert!(duration.as_secs_f32() < 2.0); // Check performance threshold
    }

    #[test]
    fn test_multiple_insertions() {
        let mut storage = DataStorage::new();
        for i in 0..100 {
            storage.insert(i, i.to_string());
        }
        
        for i in 0..100 {
            assert_eq!(storage.get(&i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_removal_of_all_elements() {
        let mut storage = DataStorage::new();
        for i in 0..10 {
            storage.insert(i, i.to_string());
        }
        
        for i in 0..10 {
            storage.remove(&i);
            assert_eq!(storage.get(&i), None); // Ensure each element is removed
        }
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let storage = std::sync::Arc::new(Mutex::new(DataStorage::new()));
        let mut handles = vec![];

        for i in 0..10 {
            let storage_clone = Arc::clone(&storage);
            let handle = thread::spawn(move || {
                let mut storage = storage_clone.lock().unwrap();
                storage.insert(i, i.to_string());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let storage = storage.lock().unwrap();
        for i in 0..10 {
            assert_eq!(storage.get(&i), Some(&i.to_string()));
        }
    }
}