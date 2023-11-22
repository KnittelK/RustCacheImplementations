use std::arch::aarch64::int8x8_t;
use std::collections::{HashMap};
use std::hash::Hash;



pub struct LRUCache<K, T> {
    recently_used_keys: Vec<K>,
    store: HashMap<K, T>,
    size: usize,
    hits: usize,
    misses: usize,
}


impl<K, V> LRUCache<K, V> where K: Eq + Hash + Clone {
    pub fn new(size: usize) -> LRUCache<K, V> {
        LRUCache {
            recently_used_keys: Vec::with_capacity(size),
            store: HashMap::new(),
            size,
            hits: 0,
            misses: 0
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // O(N) lookup
        let ret;
        if self.store.contains_key(key) {
            self.update_recently_accessed_keys(key);
            ret = self.store.get(key);
            self.hits += 1;
            ret
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        // O(N) insert
        if self.store.contains_key(&key) {
            self.update_recently_accessed_keys(&key);
        } else {
            self.insert_at_head(&key);
            self.store.insert(key, value);
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let ret;
        if self.store.contains_key(key) {
            self.hits += 1;
            self.update_recently_accessed_keys(key);
            ret = self.store.get_mut(key);
            ret
        } else {
            self.misses += 1;
            None
        }
    }

    fn hit_ratio(&self) -> f32 {
        self.hits as f32 / (self.hits + self.misses) as f32
    }

    fn miss_ratio(&self) -> f32 {
        self.misses as f32 / (self.hits + self.misses) as f32
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

#[test]
fn hit_ratio_is_point_five_when_cache_was_hit_and_missed_once() {
    let mut lru_cache = LRUCache::new(2);
    let k1 = "K1";
    let v1 = "V1";
    lru_cache.set(k1, v1);
    lru_cache.get(&k1);
    lru_cache.get(&"Key not present in cache");
    assert_eq!(lru_cache.hit_ratio(), 0.5);
}

#[test]
fn miss_ratio_is_one_and_hit_ratio_is_zero_for_a_full_miss_cache() {
    let mut lru_cache = LRUCache::new(2);
    let k1 = "K1";
    let v1 = "V1";
    lru_cache.set(k1, v1);
    lru_cache.get(&"Key not present in cache");
    assert_eq!(lru_cache.miss_ratio(), 1.);
    assert_eq!(lru_cache.hit_ratio(), 0.);
}
