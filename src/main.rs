use std::{fs::File, io::BufReader};

use axum::{Json, Router, http::StatusCode, routing::post, serve};
use serde::{Deserialize, Serialize};

mod config;
mod errors;
mod utils;

// Структура, в которую будет десериализован входящий JSON 
#[derive(Debug, Deserialize)] 
struct WriteLogRequest { 
    app_name: String, 
    log_text: String,
    priority: i8
}

// Структура, которая будет отправлена в ответ в формате JSON 
#[derive(Debug, Serialize)] 
struct ResponseStatus { 
    status: String, 
    error: String, 
}

#[tokio::main]
async fn main() {
    let cfg_path = utils::config::get_config_path().unwrap_or_else(|err| {
        eprintln!("Критическая ошибка конфигурации: {err}");
        std::process::exit(1);
    });

    let config_file = File::open(cfg_path).unwrap_or_else(|err| {
        eprintln!("Критическая ошибка файла конфигурации: {err}");
        std::process::exit(1);
    });
    let reader = BufReader::new(config_file);

    let config: config::Config = serde_json::from_reader(reader).unwrap_or_else(|err| {
        eprintln!("Критическая ошибка чтения файла конфигурации: {err}");
        std::process::exit(1);
    });

    let mut addr = "0.0.0.0:".to_owned();
    addr.push_str(&config.server.port.to_string());


    let app = Router::new().route("/log", post(create_log)); 
    println!("Server running on {:?}", addr); // Запускаем сервер 

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn create_log(Json(payload): Json<WriteLogRequest>) -> (StatusCode, Json<ResponseStatus>) {
    // Проверка входных данных 
    if payload.app_name.trim().is_empty() { 
        let response_status = ResponseStatus{
            status: "Error".to_owned(),
            error: "Имя приложения не может быть пустым".to_owned()
        };
        // Возвращаем ошибку 400, если имя пустое 
        return (StatusCode::BAD_REQUEST, Json(response_status)); 
    }

    // Форматируем строки в формат "КЛЮЧ=значение"
    let syslog_id = format!("SYSLOG_IDENTIFIER={}", &payload.app_name);
    let message = format!("MESSAGE={}", &payload.log_text);
    let priority = format!("PRIORITY={}", &payload.priority); // 6 = INFO // 3 = error

    // Собираем массив ссылок, который ожидает systemd::journal::send
    let fields: &[&str] = &[
        &syslog_id,
        &message,
        &priority,
    ];
    
    let _ = systemd::journal::send(&fields);

    let response_status = ResponseStatus{
        status: "Ok".to_owned(),
        error: "None".to_owned()
    };

    (StatusCode::OK, Json(response_status))
}
