extern crate syn;
extern crate quote;

use syn::{File, Item};
use quote::ToTokens;

fn main() {
    let code = r#"
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
            let mut storage = DataStorage::<i32, String>::new();
            storage.insert(1, "One".to_string());
            storage.insert(2, "Two".to_string());
            storage.insert(3, "Three".to_string());

            if let Some(value) = storage.get(&2) {
                println!("Value for key 2: {}", value);
            }

            if let Some(value) = storage.get(&12) {
                println!("Value for key 12 : {}", value);
            } else {
                println!("No value found for key 4");
            }
        }
    "#;

    let syntax_tree: File = syn::parse_str(code).expect("Unable to parse code");

    for item in syntax_tree.items {
        match item {
            Item::Struct(item_struct) => {
                println!("Struct: {}", item_struct.ident);
                for field in item_struct.fields {
                    println!("  Field: {:?}", field);
                }
            }
            Item::Impl(item_impl) => {
                println!("Impl: {:?}", item_impl.self_ty);
                for item in item_impl.items {
                    match item {
                        syn::ImplItem::Method(method) => {
                            println!("  Method: {}", method.sig.ident);
                        }
                        _ => {}
                    }
                }
            }
            Item::Fn(item_fn) => {
                println!("Function: {}", item_fn.sig.ident);
            }
            _ => {}
        }
    }
}
=