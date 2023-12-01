use std::collections::{HashMap};
use std::hash::Hash;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};


struct CacheEntry<V>
    where V: Clone
{
    data: V,
    ttl: usize,
    alive: Instant,
}

struct ThreadSafeTTLCache<K, V>
    where V: Clone
{
    store: RwLock<HashMap<K, CacheEntry<V>>>,
}

impl<K, V> ThreadSafeTTLCache<K, V>
    where K: Hash + Clone + Eq, V: Clone
{
    pub fn new() -> Self {
        ThreadSafeTTLCache { store: RwLock::new(HashMap::new()) }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut tombstone = false;
        let ret;
        {
            let r_store = self.store.read().expect("RWLock has been poisoned.");
            match r_store.get(key) {
                Some(cache_entry) => {
                    tombstone = cache_entry.alive.elapsed().as_secs() > cache_entry.ttl as u64;
                    if tombstone {
                        ret = None;
                    } else {
                        let v = cache_entry.data.clone();
                        ret = Some(v);
                    }
                }
                None => ret = None
            };
        }
        if tombstone {
            self.evict_entry(key);
            None
        } else {
            ret
        }
    }

    pub fn set(&self, key: &K, value: V, ttl: usize) {
        let mut w_store = self.store.write().expect("RWLock has been poisoned.");

        w_store.insert(key.clone(), CacheEntry { data: value, ttl, alive: Instant::now() });
    }

    fn evict_entry(&self, key: &K) {
        self.store.write().expect("RWLock has been poisoned.").remove(key);
    }
}

#[test]
fn multithreaded_rw() {
    fn worker_thread(id: u8, cache: Arc<ThreadSafeTTLCache<u8, &str>>) {
        match cache.get(&id) {
            Some(data) => {
                println!("GOT DATA {}", data);
            }
            None => println!("Thread id: {} found no data.", id)
        }
        cache.set(&id, "hello from the thread!", 1);
        match cache.get(&id) {
            Some(data) => {
                println!("GOT DATA AFTER SLEEPING FOR 1 SECOND {}", data);
            }
            None => println!("Thread id: {} found no data.", id)
        }
        thread::sleep(Duration::from_millis(2500));
        match cache.get(&id) {
            Some(data) => {
                println!("GOT DATA AFTER SLEEPING FOR 1.5 SECONDS {}", data);
            }
            None => println!("Thread id: {} found no data.", id)
        }
    }

    let ttl_cache = Arc::new(ThreadSafeTTLCache::new());

    let threads: Vec<_> = (0..10).map(|i| {
        let cache = Arc::clone(&ttl_cache);
        thread::spawn(move || worker_thread(i, cache))
    }).collect();

    for t in threads {
        t.join().expect("Thread panicked.")
    }
    for (_, entry) in ttl_cache.store.read().unwrap().iter() {
        println!("{:?}", entry.data);
    }
}