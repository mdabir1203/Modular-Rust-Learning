use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task;
use futures::future::try_join_all;
use anyhow::{Result, Context};
use std::thread::spawn;

#[tokio::main]
async fn main() -> Result<()> {
    // Step 1: Create a shared HashMap wrapped in Arc and Mutex for thread-safe access
    let map = Arc::new(Mutex::new(HashMap::new()));

    // Step 2: Spawn multiple asynchronous tasks
    let handles = (0..5).map(|i| {
        let map = Arc::clone(&map);
        tokio::spawn(async move {
            process_entry(i, map).await
        })
    }).collect::<Vec<_>>();

    // Step 3: Await all tasks and handle potential errors
    try_join_all(handles).await?;

    // Step 4: Display final state of the HashMap
    let final_map = map.lock().await;
    println!("Final HashMap: {:?}", *final_map);

    Ok(())
}

async fn process_entry(i: i32, map: Arc<Mutex<HashMap<i32, i32>>>) -> Result<()> {
    let mut map = map.lock().await;
    match map.entry(i) {
        std::collections::hash_map::Entry::Occupied(_) => {
            println!("Key {} already exists!", i);
        },
        std::collections::hash_map::Entry::Vacant(e) => {
            e.insert(i * 2);
            println!("Inserted: {} => {}", i, i * 2);
        }
    }
    Ok(())
}