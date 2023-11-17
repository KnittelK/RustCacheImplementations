
struct Node<K, V> {
    key: K,
    value: V
}

struct VecLRUCache<K, V>{
    store: Vec<Node<K, V>>,
    size: usize
}

impl<K,V> VecLRUCache<K, V> where K: Clone + Eq , V: Clone{
    fn new(size: usize) -> Self {
        assert!(size > 0);

        VecLRUCache {size, store: Vec::with_capacity(size)}
    }

    fn insert(&mut self, key: K, value: V) {
        // O(1) insert
        let node = Node {key, value};
        if self.store.len() == self.size {
            self.store.pop();
        }
        self.store.insert(0, node);
    }

    fn get(&mut self, key: &K) -> Option<V> {
        // O(N) lookup
        let mut present = false;
        let mut present_idx = 0;
        for (idx, node) in self.store.iter().enumerate() {
            if (*node).key == *key {
                present = true;
                present_idx = idx;
                break;
            }
        }
        if present {
            // update the location of the looked up node to the front of the cache.
            let ret_val = self.store.remove(present_idx).value;
            self.store.insert(0, Node {key: key.clone(), value: ret_val.clone()});
            Some(ret_val)
        } else {
            None
        }
    }
}

#[test]
fn test_lru_cache() {
    let mut lru_cache = VecLRUCache::new(2);
    let key1 = "some Key 1";
    let val1 = "some Value";
    let key2 = "some Key 2";
    lru_cache.insert(key1, "some Value");
    lru_cache.insert(key2, "some Value");
    assert_eq!(val1, lru_cache.get(&key1).unwrap());
}

#[test]
fn test_lru_cache_size() {
    let mut lru_cache = VecLRUCache::new(2);
    let k1 = "K1";
    let k2 = "K2";
    let k3 = "K3";
    let v1 = "V1";
    let v2 = "V2";
    let v3 = "V3";
    lru_cache.insert(k1, v1);
    lru_cache.insert(k2, v2);
    lru_cache.insert(k3, v3);
    assert!(lru_cache.get(&k1).is_none());
}

#[test]
fn test() {
    #[derive(Clone)]
    struct MyValue {
        id: u32,
        name: &'static str,
    };

    // Create an empty cache, then insert some items.
    let k1 = "k1";
    let k2 = "k2";
    let k3 = "k3";
    let mut cache = VecLRUCache::new(3);
    cache.insert(k1, MyValue { id: 1, name: "Mercury" });
    cache.insert(k2, MyValue { id: 2, name: "Venus" });
    cache.insert(k3, MyValue { id: 3, name: "Earth" });

    // Use the `get` method to retrieve an item from the cache.
    // This also "touches" the item, marking it most-recently-used.
    let item = cache.get(&k1);
    assert_eq!(item.unwrap().name, "Mercury");

    // If the cache is full, inserting a new item evicts the least-recently-used item:
    let k4 = "k4";
    cache.insert(k4, MyValue { id: 4, name: "Mars" });
    assert!(cache.get(&k2).is_none());
}