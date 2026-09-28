/// Ошибка файла конфигурации
#[derive(Debug)]
pub enum FileConfigError {
    // NotFound(PathBuf),
    // Unreadable(PathBuf, io::Error),
    HomeDirNotAvailable,
    NotAvailable,
}

impl std::fmt::Display for FileConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // FileConfigError::NotFound(path) => write!(f, "Конфиг не найден в путях: {:?}", path),
            // FileConfigError::Unreadable(p, e) => write!(f, "Не удалось прочитать {:?}: {}", p, e),
            FileConfigError::HomeDirNotAvailable => write!(f, "Каталог 'HOME' не доступен"),
            FileConfigError::NotAvailable => write!(f, "Файл не доступен или не существует"),
        }
    }
}
