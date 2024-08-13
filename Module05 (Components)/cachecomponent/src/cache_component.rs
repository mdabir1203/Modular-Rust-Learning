// Import the necessary modules from the standard library
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;


// Defined a trait (interface) for a cache that can get and set values
pub trait GetCache<K, V> {
    // Getting a value from the cache by its key
    fn get(&self, key: &K) -> Option<&V>;
    // Setting a value in the cache with a given key
    fn set(&mut self, key: K, value: V);
}

// Defined a struct for an in-memory cache
pub struct InMemoryCache<K, V> {
    // The underlying data storage is a HashMap
    data: HashMap<K, V>,
}

// Implemented methods for the InMemoryCache struct
impl<K, V> InMemoryCache<K, V> {
    // Creating a new, empty in-memory cache
    pub fn new() -> Self {
        InMemoryCache { data: HashMap::new() }
    }
}

// Implemented the GetCache trait for InMemoryCache
impl<K: Eq + Hash, V> GetCache<K, V> for InMemoryCache<K, V> {
    // Getting a value from the cache by its key
    fn get(&self, key: &K) -> Option<&V> {
        // Delegated to the underlying HashMap
        self.data.get(key)
    }
    // Setting a value in the cache with a given key
    fn set(&mut self, key: K, value: V) {
        // Delegated to the underlying HashMap
        self.data.insert(key, value);
    }
}

// Defined a struct for a cache that can use different strategies
pub struct Cache<K, V, T: GetCache<K, V>> {
    // The underlying cache strategy
    strategy: T,
    _marker: PhantomData<(K, V)>,
}

// Implement methods for the Cache struct
impl<K, V, T: GetCache<K, V>> Cache<K, V, T> {
    // Create a new cache with a given strategy
    pub fn new(strategy: T) -> Self {
        Cache { strategy,
            // PhantomData is a marker type that captures the type parameters
            _marker: PhantomData,
         }
    }

    // Get a value from the cache by its key
    pub fn get(&self, key: &K) -> Option<&V> {
        // Delegate to the underlying strategy
        self.strategy.get(key)
    }

    // Set a value in the cache with a given key
    pub fn set(&mut self, key: K, value: V) {
        // Delegate to the underlying strategy
        self.strategy.set(key, value);
    }
}