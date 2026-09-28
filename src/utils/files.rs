use std::{fs, io::Error, path::Path};

/// Вспомогательная функция: проверяет, что перед нами именно файл и он доступен для чтения.
pub fn is_file_readable<P: AsRef<Path>>(path: P) -> Result<bool, Error> {
    fs::metadata(path.as_ref()).map(|m| m.is_file()) // Игнорируем директории с таким именем .unwrap_or(false)
}
