use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::fs;
use std::io;
use std::thread;

struct ConfigManager {
    config: HashMap<String, String>,
}

impl ConfigManager {
    fn new() -> Result<Self, io::Error> {
        let config_str = fs::read_to_string("src/config.toml")?;
        let config = parse_config(&config_str);
        Ok(ConfigManager { config })
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    fn set(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    fn save(&self) -> Result<(), io::Error> {
        let config_str = format_config(&self.config);
        fs::write("config.toml", config_str)
    }
}

fn get_config() -> Arc<Mutex<ConfigManager>> {
    static mut SINGLETON: Option<Arc<Mutex<ConfigManager>>> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            match ConfigManager::new() {
                Ok(config) => {
                    SINGLETON = Some(Arc::new(Mutex::new(config)));
                }
                Err(e) => {
                    eprintln!("Failed to initialize ConfigManager: {}", e);
                    std::process::exit(1);
                }
            }
        });

        SINGLETON.clone().unwrap()
    }
}

fn parse_config(config_str: &str) -> HashMap<String, String> {
    config_str
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn format_config(config: &HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = config.keys().collect();
    keys.sort();
    keys.iter()
        .map(|k| format!("{} = {}", k, config.get(*k).unwrap()))
        .collect::<Vec<String>>()
        .join("\n")
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let config = get_config();
                let mut config = config.lock().unwrap();

                let db_url = config.get("database_url").cloned().unwrap_or_default();
                println!("Thread {} read database_url: {}", i, db_url);

                let new_value = format!("new_value_from_thread_{}", i);
                config.set(format!("key_from_thread_{}", i), new_value.clone());
                println!("Thread {} set new value: {}", i, new_value);

                thread::sleep(std::time::Duration::from_millis(100));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let config = get_config();
    let config = config.lock().unwrap();
    config.save()?;
    println!("Configuration saved.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config_str = "key1 = value1\nkey2 = value2\nkey3 = value3";
        let config = parse_config(config_str);
        assert_eq!(config.len(), 3);
        assert_eq!(config.get("key1"), Some(&"value1".to_string()));
        assert_eq!(config.get("key2"), Some(&"value2".to_string()));
        assert_eq!(config.get("key3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_format_config() {
        let mut config = HashMap::new();
        config.insert("key1".to_string(), "value1".to_string());
        config.insert("key2".to_string(), "value2".to_string());
        config.insert("key3".to_string(), "value3".to_string());

        let config_str = format_config(&config);
        assert_eq!(config_str, "key1 = value1\nkey2 = value2\nkey3 = value3");
    }
}