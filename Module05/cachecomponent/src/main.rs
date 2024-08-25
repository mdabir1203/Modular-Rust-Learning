pub mod cache_component;

use cache_component::{Cache, InMemoryCache};


fn main() {
    let mut cache = Cache::new(InMemoryCache::new());

    cache.set("key1".to_string(), 51);
    cache.set("key2".to_string(), 71);

    if let Some(value) = cache.get(&"key1".to_string())
    {
        println!("Cached Value: {}", value);
    } else {
        println!("No value found in cache");
    }

//    println!("Value for key1: {:?}", cache.get(&"key1".to_string()));
//    println!("Value for key2: {:?}", cache.get(&"key2".to_string()));
//    println!("Value for key3: {:?}", cache.get(&"key3".to_string()));


}