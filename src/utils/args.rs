use std::env;

/// Метод возвращает значение аргумента по его имени или None
///
/// #### Arguments
///
/// * `name` - имя аргумента (полностью, включая значи '-')
///
/// #### Returns
///
/// Значение аргумента или None, тип Optional\<String\>
pub fn get_value_by_name(name: &str) -> Option<String> {
    let mut args = env::args_os().skip(1);

    while let Some(arg) = args.next() {
        if arg.to_str() == Some(name) {
            if let Some(next_arg) = args.next() {
                if let Ok(path_str) = next_arg.into_string() {
                    return Some(path_str);
                }
            }
        }
    }

    None
}
