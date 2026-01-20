use std::fs;
use std::path::PathBuf;

pub struct CacheManager;

impl CacheManager {
    fn get_cache_dir() -> PathBuf {
        let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("mizu");
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        path
    }

    pub fn write(key: &str, value: &str) {
        let path = Self::get_cache_dir().join(key);
        let _ = fs::write(path, value);
    }

    pub fn read(key: &str) -> Option<String> {
        let path = Self::get_cache_dir().join(key);
        fs::read_to_string(path).ok()
    }
}
