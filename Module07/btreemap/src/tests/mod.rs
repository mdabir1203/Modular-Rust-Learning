#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_insertion() {
        let mut btree = BTree::new(3);
        btree.insert(10, "ten");
        btree.insert(20, "twenty");
        assert_eq!(btree.search(10), Some(&"ten"));
    }
    
    #[test]
    fn test_deletion() {
        let mut btree = BTree::new(3);
        btree.insert(10, "ten");
        btree.insert(20, "twenty");
        btree.delete(10);
        assert_eq!(btree.search(10), None);
    }

    #[test]
    fn test_performance() {
        let mut btree = BTree::new(4);
        let start = Instant::now();
        
        for i in 0..10000 {
            btree.insert(i, i.to_string());
        }
        
        let duration = start.elapsed();
        println!("Insertion took {:?}", duration);
        
        assert!(duration.as_secs_f32() < 2.0); // Check performance threshold
    }
}
