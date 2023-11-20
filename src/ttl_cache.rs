use std::collections::HashMap;
use std::hash::Hash;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Node<V>
    where V: Clone
{
    data: V,
    ttl: usize,
    alive: Instant,
}

struct TTLCache<K, V>
{
    store: HashMap<K, V>,
}

impl<K, V> TTLCache<K, Node<V>>
    where K: Hash + Clone + Eq, V: Clone
{
    pub fn new() -> Self {
        TTLCache { store: HashMap::new() }
    }

    pub fn set(&mut self, key: K, value: V, ttl: usize) {
        // ttl of 0 means keep the item forever
        let ttl = match ttl {
            0 => usize::MAX,
            _ => ttl
        };
        let node = Node { data: value, ttl, alive: Instant::now() };
        self.store.insert(key, node);
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if self.store.contains_key(&key) {
            let node = self.store.get(&key).unwrap();
            let tombstone = node.alive.elapsed().as_secs() > node.ttl as u64;
            if tombstone {
                self.store.remove(&key);
                None
            } else {
                Some(node.data.clone())
            }
        } else {
            None
        }
    }
}

#[test]
fn ttl_cache_set_and_get_work(){
    let k1 = "k1";
    let k2 = "k2";
    let k3 = "k3";
    let v1 = "v1";
    let v2 = "v2";
    let v3 = "v3";

    let mut ttl_cache = TTLCache::new();
    ttl_cache.set(k1, v1, 300);
    ttl_cache.set(k2, v2, 300);
    ttl_cache.set(k3, v3, 300);

    assert_eq!(ttl_cache.get(&k1).unwrap(), v1);
}

#[test]
fn ttl_cache_getting_expired_entry_yields_none(){
    let k1 = "k1";
    let v1 = "v1";


    let mut ttl_cache = TTLCache::new();
    ttl_cache.set(k1, v1, 1);
    sleep(Duration::from_millis(2000));
    assert_eq!(ttl_cache.get(&k1).is_none(), true);
}

#[test]
fn size_of_cache_reflects_when_an_entity_is_deleted(){
    let k1 = "k1";
    let v1 = "v1";


    let mut ttl_cache = TTLCache::new();
    ttl_cache.set(k1, v1, 1);
    assert_eq!(ttl_cache.store.len(), 1);
    sleep(Duration::from_millis(2000));
    assert_eq!(ttl_cache.get(&k1).is_none(), true);
    assert_eq!(ttl_cache.store.len(), 0);
}
