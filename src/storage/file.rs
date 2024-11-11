use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::fmt::Display;
use std::str::FromStr;

/// Gère la persistance des données du cache dans un fichier
/// 
/// Cette structure fournit des méthodes statiques pour sauvegarder et charger
/// les données du cache depuis un fichier texte.
/// 
/// # Exemples
/// 
/// ```
/// use lru_cache::storage::file::FileStorage;
/// 
/// // Sauvegarde des données
/// let data = vec![
///     (String::from("key1"), String::from("value1")),
///     (String::from("key2"), String::from("value2")),
/// ];
/// FileStorage::save("cache.txt", 2, &data).unwrap();
/// 
/// // Chargement des données
/// let (capacity, loaded_data) = FileStorage::load::<String, String>("cache.txt").unwrap();
/// assert_eq!(capacity, 2);
/// assert_eq!(loaded_data.len(), 2);
/// 
/// // Nettoyage
/// std::fs::remove_file("cache.txt").unwrap();
/// ```
/// 
/// Exemple avec des types numériques :
/// ```
/// use lru_cache::storage::file::FileStorage;
/// 
/// // Sauvegarde des données numériques
/// let data = vec![(1, 100), (2, 200)];
/// FileStorage::save("numbers.txt", 2, &data).unwrap();
/// 
/// // Chargement des données numériques
/// let (capacity, loaded_data) = FileStorage::load::<i32, i32>("numbers.txt").unwrap();
/// assert_eq!(capacity, 2);
/// assert_eq!(loaded_data, vec![(1, 100), (2, 200)]);
/// 
/// // Nettoyage
/// std::fs::remove_file("numbers.txt").unwrap();
/// ```
pub struct FileStorage;

impl FileStorage {
    /// Sauvegarde les données du cache dans un fichier
    /// 
    /// # Arguments
    /// 
    /// * `path` - Le chemin du fichier où sauvegarder les données
    /// * `capacity` - La capacité du cache
    /// * `data` - Les paires clé-valeur à sauvegarder
    /// 
    /// # Format du fichier
    /// 
    /// La première ligne contient la capacité du cache.
    /// Chaque ligne suivante contient une paire clé-valeur séparée par ';'.
    /// 
    /// # Exemple
    /// 
    /// ```
    /// use lru_cache::storage::file::FileStorage;
    /// 
    /// let data = vec![("key1", 42), ("key2", 84)];
    /// FileStorage::save("test.txt", 2, &data).unwrap();
    /// 
    /// // Le fichier contiendra :
    /// // 2
    /// // key1;42
    /// // key2;84
    /// 
    /// std::fs::remove_file("test.txt").unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// Retourne une erreur si :
    /// - Le fichier ne peut pas être créé ou ouvert
    /// - L'écriture dans le fichier échoue
    pub fn save<K: Display, V: Display>(path: &str, capacity: usize, data: &[(K, V)]) -> io::Result<()> {
        let mut content = String::new();
        content.push_str(&format!("{}\n", capacity));
        
        for (key, value) in data {
            content.push_str(&format!("{};{}\n", key, value));
        }
        
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
        writer.flush()
    }

    /// Charge les données du cache depuis un fichier
    /// 
    /// # Arguments
    /// 
    /// * `path` - Le chemin du fichier à charger
    /// 
    /// # Returns
    /// 
    /// Retourne un tuple contenant :
    /// - La capacité du cache
    /// - Un vecteur des paires clé-valeur chargées
    /// 
    /// # Exemple
    /// 
    /// ```
    /// use lru_cache::storage::file::FileStorage;
    /// use std::fs::write;
    /// 
    /// // Création d'un fichier de test
    /// write("load_test.txt", "2\nkey1;42\nkey2;84\n").unwrap();
    /// 
    /// // Chargement des données
    /// let (capacity, data) = FileStorage::load::<String, i32>("load_test.txt").unwrap();
    /// assert_eq!(capacity, 2);
    /// assert_eq!(data[0], (String::from("key1"), 42));
    /// 
    /// // Nettoyage
    /// std::fs::remove_file("load_test.txt").unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// Retourne une erreur si :
    /// - Le fichier ne peut pas être ouvert
    /// - La lecture du fichier échoue
    /// - Le format du fichier est invalide
    /// 
    /// # Note
    /// 
    /// Les entrées qui ne peuvent pas être parsées sont silencieusement ignorées.
    pub fn load<K: FromStr, V: FromStr>(path: &str) -> io::Result<(usize, Vec<(K, V)>)> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        
        let mut lines = content.lines();
        let capacity = lines.next()
            .and_then(|l| l.parse().ok())
            .unwrap_or(0);
            
        let mut data = Vec::new();
        for line in lines {
            if let Some((key_str, value_str)) = line.split_once(';') {
                if let (Ok(key), Ok(value)) = (K::from_str(key_str), V::from_str(value_str)) {
                    data.push((key, value));
                }
            }
        }
        
        Ok((capacity, data))
    }
}