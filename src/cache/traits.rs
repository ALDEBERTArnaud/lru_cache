/// Définit les opérations de base d'un cache
pub trait CacheStorage<K, V> {
    /// Récupère une valeur du cache
    fn get(&mut self, key: &K) -> Option<&V>;
    /// Insère une valeur dans le cache
    fn put(&mut self, key: K, value: V);
}

/// Définit les opérations de persistance d'un cache
pub trait PersistentStorage<K, V> {
    /// Sauvegarde le cache dans un fichier
    fn save_to_file(&self, path: &str) -> std::io::Result<()>;
    /// Charge le cache depuis un fichier
    fn load_from_file(path: &str, capacity: usize) -> std::io::Result<Self>
    where
        Self: Sized;
}