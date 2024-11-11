use super::traits::{CacheStorage, PersistentStorage};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

/// Cache LRU (Least Recently Used) qui stocke les éléments les plus récemment utilisés
/// 
/// # Examples
/// ```
/// use lru_cache::Cache;
/// use lru_cache::cache::traits::CacheStorage;
/// 
/// let mut cache = Cache::new(3);
/// cache.put("key1", 42);
/// assert_eq!(cache.get(&"key1"), Some(&42));
/// ```
#[derive(Debug)]
pub struct Cache<K, V> {
    capacity: usize,
    storage: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Clone + Eq + Hash, V> Cache<K, V> {
    /// Crée un nouveau cache avec la capacité spécifiée
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            storage: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }

    fn update_order(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
            self.order.push(key.clone());
        }
    }
}

impl<K: Clone + Eq + Hash, V> CacheStorage<K, V> for Cache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V> {
        if self.storage.contains_key(key) {
            self.update_order(key);
            self.storage.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.storage.contains_key(&key) {
            self.storage.insert(key.clone(), value);
            self.update_order(&key);
        } else {
            if self.storage.len() >= self.capacity {
                if let Some(lru_key) = self.order.first().cloned() {
                    self.storage.remove(&lru_key);
                    self.order.remove(0);
                }
            }
            self.storage.insert(key.clone(), value);
            self.order.push(key);
        }
    }
}

impl<K: Clone + Eq + Hash + Display + FromStr, V: Display + FromStr> PersistentStorage<K, V> for Cache<K, V> {
    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let data: Vec<_> = self.order.iter()
            .filter_map(|k| self.storage.get(k).map(|v| (k, v)))
            .collect();
        crate::storage::file::FileStorage::save(path, self.capacity, &data)
    }

    fn load_from_file(path: &str, capacity: usize) -> std::io::Result<Self> {
        let (_, data) = crate::storage::file::FileStorage::load(path)?;
        let mut cache = Cache::new(capacity);
        for (key, value) in data {
            cache.put(key, value);
        }
        Ok(cache)
    }
}