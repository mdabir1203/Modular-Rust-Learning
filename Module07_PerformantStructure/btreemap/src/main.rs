use std::collections::BTreeMap;

pub struct DataStorage<K, V> {
    store: BTreeMap<K, V>,
}

impl<K: Ord, V> DataStorage<K, V> {
    pub fn new() -> Self {
        DataStore {
            store: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
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