use crate::node::Node;

pub struct BTree<K, V> {
    root: Node<K, V>,
    degree: usize,
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new(degree: usize) -> Self {
        assert!(degree >= 2, "Degree must be at least 2");
        Self {
            root: Node::new(true),
            degree,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.root.keys_len() == 2 * self.degree - 1 {
            let mut new_root = Node::new(false);
            let mut old_root = std::mem::replace(&mut self.root, Node::new(true));
            let (median_key, right_child) = old_root.split();
            new_root.keys.push(median_key);
            new_root.children = Some(vec![Box::new(old_root), right_child]);
            self.root = new_root;
        }
        self.root.insert(key, value, self.degree - 1);
    }

    pub fn search<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.root.search(key)
    }
}