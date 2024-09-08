use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use json_web_token::payload::Payload;
use log::error;
use tokio::sync::Mutex;

include!("../struct/todo.rs");

// Обработчик для POST-запроса
async fn post_todo(
    Extension(state): Extension<Arc<Mutex<AppState>>>, // Состояние с клиентом БД
    Json(payload): Json<Todo>, // Данные в формате JSON
) {
    let todo = payload;

    // Логируем приходящий JSON
    info!("Received JSON: {:?}", todo);

    let mut state = state.lock().await; // Получаем блокировку состояния
    let client = &state.db_client; // Используем ссылку на клиента

    let query = "INSERT INTO todo (username, title, completed, content) VALUES ($1, $2, $3, $4)";

    match client.execute(query, &[&todo.username, &todo.title, &todo.completed, &todo.content]).await {
        Ok(_) => {
            info!("Todo успешно добавлено");
            (axum::http::StatusCode::CREATED, "Todo создано").into_response();
        }
        Err(e) => {
            error!("Ошибка при добавлении в базу: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Ошибка при добавлении Todo").into_response();
        }
    }
}