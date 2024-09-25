use std::collections::HashMap;
pub mod colors;

// A structure to represent our key-value store
struct SearchEngineKV {
    store: HashMap<String, Vec<String>>,
}

impl SearchEngineKV {
    // Create a new empty store
    fn new() -> SearchEngineKV {
        SearchEngineKV {
            store: HashMap::new(),
        }
    }

    // Insert a URL for a given keyword
    fn insert(&mut self, key: String, value: String) {
        self.store.entry(key).or_insert_with(Vec::new).push(value);
    }

    // Retrieve URLs for a given keyword
    fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.store.get(key)
    }

    // Remove a URL from a keyword entry
    fn remove_url(&mut self, key: &str, url: &str) {
        if let Some(urls) = self.store.get_mut(key) {
            urls.retain(|u| u != url);
            if urls.is_empty() {
                self.store.remove(key);
            }
        }
    }
}

fn main() {
    let mut engine = SearchEngineKV::new();

    // Adding some sample data
    engine.insert("Cplusplus".to_string(), "https://cplusplus.com/".to_string());
    engine.insert("rust programming".to_string(), "https://doc.rust-lang.org".to_string());
    engine.insert("web development".to_string(), "https://developer.mozilla.org".to_string());

    // Retrieving data
    if let Some(urls) = engine.get("rust programming") {
        println!("URLs for 'rust programming': {:?}", urls);
    }

    // Removing a URL
    engine.remove_url("web development", "https://developer.mozilla.org");
    
    // Check if URL was removed // fix the line below 
    if let Some(urls) = engine.get("web development") {
        println!("URLs for 'web development' after removal: {:?}", urls);
    } else {
        println!("No URLs found for 'web development' after removal");
    }

}