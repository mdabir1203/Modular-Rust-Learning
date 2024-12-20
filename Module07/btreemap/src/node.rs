use std::cmp::Ord;

pub struct Node<K, V> {
    pub keys: Vec<K>,
    pub values: Option<Vec<V>>, // Only for leaf nodes
    pub children: Option<Vec<Box<Node<K, V>>>>, // Only for internal nodes
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn new(is_leaf: bool) -> Self {
        Self {
            keys: Vec::new(),
            values: if is_leaf { Some(Vec::new()) } else { None },
            children: if !is_leaf { Some(Vec::new()) } else { None },
        }
    }

    pub fn keys_len(&self) -> usize {
        self.keys.len()
    }

    pub fn split(&mut self) -> (K, Box<Node<K, V>>) {
        let mid = self.keys.len() / 2;
        let median_key = self.keys[mid].clone();
        
        let mut right_child = Box::new(Node::new(self.is_leaf()));
        right_child.keys = self.keys.split_off(mid + 1);
        
        if let Some(ref mut v) = self.values {
            right_child.values = Some(v.split_off(mid + 1));
        }
        
        if let Some(ref mut c) = self.children {
            right_child.children = Some(c.split_off(mid + 1));
        }
        
        self.keys.pop(); // Remove the median key from the left node
        
        (median_key, right_child)
    }
    
    pub fn insert(&mut self, key: K, value: V, degree: usize) {
        let pos = self.keys.binary_search(&key).unwrap_or_else(|e| e);

        if self.is_leaf() {
            self.keys.insert(pos, key);
            self.values.as_mut().unwrap().insert(pos, value);
        } else {
            let child = &mut self.children.as_mut().unwrap()[pos];
            child.insert(key, value, degree);
            
            if child.keys_len() > degree {
                let (median_key, right_child) = child.split();
                self.keys.insert(pos, median_key);
                self.children.as_mut().unwrap().insert(pos + 1, right_child);
            }
        }
    }

    pub fn search<'a>(&'a self, key: &K) -> Option<&'a V> {
        let pos = self.keys.binary_search(key).unwrap_or_else(|e| e);

        if pos < self.keys.len() && &self.keys[pos] == key {
            self.values.as_ref().unwrap().get(pos)
        } else if self.is_leaf() {
            None
        } else {
            self.children.as_ref().unwrap()[pos].search(key)
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}