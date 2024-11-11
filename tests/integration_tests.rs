use lru_cache::Cache;
use lru_cache::cache::traits::{CacheStorage, PersistentStorage};
use lru_cache::storage::file::FileStorage;
use std::fs;

#[test]
fn test_lru_cache_basic() {
    let mut cache = Cache::new(3);
    cache.put("A", String::from("value_a"));
    cache.put("B", String::from("value_b"));
    cache.put("C", String::from("value_c"));
    cache.put("D", String::from("value_d"));

    assert_eq!(cache.get(&"A"), None);
    assert_eq!(cache.get(&"D"), Some(&String::from("value_d")));
}

#[test]
fn test_lru_cache_generic() {
    let mut cache = Cache::new(2);
    cache.put(1, 100);
    cache.put(2, 200);
    cache.put(3, 300);

    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&200));
}

#[test]
fn test_persistent_cache() {
    let path = "test_cache.txt";
    {
        let mut cache = Cache::new(2);
        cache.put(String::from("X"), String::from("test"));
        cache.save_to_file(path).unwrap();
    }

    let mut cache = Cache::<String, String>::load_from_file(path, 2).unwrap();
    assert_eq!(cache.get(&String::from("X")), Some(&String::from("test")));
    
    fs::remove_file(path).unwrap();
}

#[test]
fn test_lru_eviction_order() {
    let mut cache = Cache::new(3);
    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3);
    
    assert_eq!(cache.get(&"B"), Some(&2));
    
    cache.put("D", 4);
    
    assert_eq!(cache.get(&"A"), None);
    assert_eq!(cache.get(&"B"), Some(&2));
    assert_eq!(cache.get(&"C"), Some(&3));
    assert_eq!(cache.get(&"D"), Some(&4));
}

#[test]
fn test_file_storage_basic() {
    let path = "test_storage.txt";
    let data = vec![
        (String::from("key1"), String::from("value1")),
        (String::from("key2"), String::from("value2")),
    ];
    
    FileStorage::save(path, 2, &data).unwrap();
    let (capacity, loaded_data) = FileStorage::load::<String, String>(path).unwrap();
    
    assert_eq!(capacity, 2);
    assert_eq!(loaded_data, data);
    
    fs::remove_file(path).unwrap();
}

#[test]
fn test_file_storage_numeric() {
    let path = "test_numbers.txt";
    let data = vec![(1, 100), (2, 200)];
    
    FileStorage::save(path, 2, &data).unwrap();
    let (capacity, loaded_data) = FileStorage::load::<i32, i32>(path).unwrap();
    
    assert_eq!(capacity, 2);
    assert_eq!(loaded_data, data);
    
    fs::remove_file(path).unwrap();
}