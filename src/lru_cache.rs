use std::collections::{HashMap};
use std::hash::Hash;

struct LRUCache<K, T> {
    recently_used_keys: Vec<K>,
    store: HashMap<K, T>,
    size: usize,
}


impl<K, V> LRUCache<K, V> where K: Eq + Hash + Clone{
    pub fn new(size: usize) -> LRUCache<K, V> {
        LRUCache { recently_used_keys: Vec::with_capacity(size), store: HashMap::new(), size }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // O(N) lookup
        let ret;
        if self.store.contains_key(key){
            self.update_recently_accessed_keys(key);
            ret = self.store.get(key);
            ret
        } else {
            None
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        // O(N) insert
        if self.store.contains_key(&key){
            self.update_recently_accessed_keys(&key);
        } else {
            self.insert_at_head(&key);
            self.store.insert(key, value);
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let ret;
        if self.store.contains_key(key){
            self.update_recently_accessed_keys(key);
            ret = self.store.get_mut(key);
            ret
        } else {
            None
        }
    }

    fn insert_at_head(&mut self, key: &K) {
        if self.recently_used_keys.len() == self.size {
            self.recently_used_keys.pop();
        }
        self.recently_used_keys.insert(0, (*key).clone());
    }

    fn update_recently_accessed_keys(&mut self, key: &K) {
        for (idx, element) in self.recently_used_keys.iter().enumerate() {
            if element == key {
                self.recently_used_keys.remove(idx);
                break;
            }
        }
        self.recently_used_keys.insert(0, (*key).clone());
        if self.recently_used_keys.len() > self.size {
            self.recently_used_keys.pop();
            self.store.remove(key);
        }
    }
}


#[test]
fn test_lru_cache() {
    let mut lru_cache = LRUCache::new(2);
    let key1 = "some Key 1";
    let val1 = "some Value";
    let key2 = "some Key 2";
    lru_cache.set(key1, "some Value");
    lru_cache.set(key2, "some Value");
    assert_eq!(val1, *(lru_cache.get(&key1).unwrap()));
}

#[test]
fn test_lru_cache_size() {
    let mut lru_cache = LRUCache::new(2);
    let k1 = "K1";
    let k2 = "K2";
    let k3 = "K3";
    let v1 = "V1";
    let v2 = "V2";
    let v3 = "V3";
    lru_cache.set(k1, v1);
    lru_cache.set(k2, v2);
    lru_cache.set(k3, v3);
    assert!(lru_cache.get(&k1).is_none());
}

//
// #[test]
// fn test_mutable_contents() {
//     let mut cache = LRUCache::new(2);
//     cache.set("apple", 3);
//     cache.set("banana", 2);
//
//     assert_eq!(*cache.get(&"apple").unwrap(), 3);
//     assert_eq!(*cache.get(&"banana").unwrap(), 2);
//     assert!(cache.get(&"pear").is_none());
//
//     assert_eq!(cache.set("banana", 4), Some(2));
//     assert_eq!(cache.set("pear", 5), None);
//
//     assert_eq!(*cache.get(&"pear").unwrap(), 5);
//     assert_eq!(*cache.get(&"banana").unwrap(), 4);
//     assert!(cache.get(&"apple").is_none());
//
//     {
//         let v = cache.get_mut(&"banana").unwrap();
//         *v = 6;
//     }
//
//     assert_eq!(*cache.get(&"banana").unwrap(), 6);
// }