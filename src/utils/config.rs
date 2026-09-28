use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{errors::config::FileConfigError, utils};

/// Находит путь к файлу конфигурации, проверяя источники в порядке приоритета:
/// 1. Аргумент командной строки `-c <path>`.
/// 2. Пользовательский каталог согласно спецификации XDG Base Directory (`$XDG_CONFIG_HOME` или `$HOME/.config`).
/// 3. Системный каталог `/etc/sys_logger_ljd/config.json`.
///
/// #### Returns
///
/// Возвращает первый найденный доступный для чтения файл в виде `Result<String>` или ошибку.
pub fn get_config_path() -> Result<String, FileConfigError> {
    // 1. Проверка аргументов командной строки.
    // Ищем флаг -c и берем следующий за ним аргумент как путь.
    if let Some(cfg_path) = utils::args::get_value_by_name("-c") {
        let p = Path::new(&cfg_path);

        return match utils::files::is_file_readable(p) {
            Ok(_) => Ok(cfg_path),
            Err(_e) => Err(FileConfigError::NotAvailable),
        };
    }

    // 2. Сборка пользовательского пути
    let user_path = build_user_path().map_err(|_| FileConfigError::HomeDirNotAvailable)?;

    // 3. Проверяем пользовательский путь
    match utils::files::is_file_readable(&user_path) {
        Ok(_) => return Ok(user_path.to_string_lossy().into_owned()),
        Err(_user_err) => {
            // 4. Только если домашний конфиг недоступен, пробуем системный
            let system_path = Path::new("/etc/sys_logger_ljd/config.json");

            match utils::files::is_file_readable(system_path) {
                Ok(_) => return Ok(system_path.to_string_lossy().into_owned()),
                Err(_sys_err) => {
                    return Err(FileConfigError::NotAvailable);
                }
            }
        }
    }
}

fn build_user_path() -> Result<PathBuf, env::VarError> {
    let mut path = dirs::config_dir().ok_or(env::VarError::NotPresent)?;
    path.push("sys_logger_ljd1");
    path.push("config.json");

    Ok(path)
}
