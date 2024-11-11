//! # LRU Cache
//! 
//! Une implémentation d'un cache LRU (Least Recently Used) en Rust.
//! 
//! ## Caractéristiques
//! 
//! - Cache générique supportant différents types de clés et valeurs
//! - Persistance des données dans un fichier
//! - Implémentation basée sur les traits pour plus de flexibilité
//! 
//! ## Exemple d'utilisation
//! 
//! ```rust
//! use lru_cache::Cache;
//! use lru_cache::cache::traits::CacheStorage;
//! 
//! // Création d'un cache de taille 3
//! let mut cache = Cache::new(3);
//! 
//! // Ajout d'éléments
//! cache.put("A", String::from("value_a"));
//! cache.put("B", String::from("value_b"));
//! cache.put("C", String::from("value_c"));
//! 
//! // Vérification que tous les éléments sont présents
//! assert_eq!(cache.get(&"A"), Some(&String::from("value_a")));
//! assert_eq!(cache.get(&"B"), Some(&String::from("value_b")));
//! assert_eq!(cache.get(&"C"), Some(&String::from("value_c")));
//! 
//! // Ajout de D qui dépasse la capacité
//! cache.put("D", String::from("value_d"));
//! 
//! // A est éjecté car c'était le moins récemment utilisé
//! assert_eq!(cache.get(&"A"), None);
//! assert_eq!(cache.get(&"D"), Some(&String::from("value_d")));
//! ```
//! 
//! ## Persistance
//! 
//! ```rust
//! use lru_cache::Cache;
//! use lru_cache::cache::traits::{CacheStorage, PersistentStorage};
//! use std::fs;
//! 
//! let path = "cache.txt";
//! let mut cache = Cache::new(3);
//! cache.put(String::from("key"), String::from("value"));
//! cache.save_to_file(path).unwrap();
//! 
//! // Nettoyage
//! fs::remove_file(path).unwrap();
//! ```

pub mod cache;
pub mod storage;

pub use cache::lru::Cache;