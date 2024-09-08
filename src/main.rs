use log::{info};
use chrono::Local;
use tokio_postgres::{NoTls, Client, Error};


include!("routers/get_todo_router.rs");

// Структура для хранения состояния приложения
pub struct AppState {
    db_client: Client,
}

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    // Подключаемся к базе данных и создаём AppState
    let app_state = check_db().await.expect("Не удалось подключиться к БД");

    info!("{:20} {}", "application running", Local::now());

    // Передаём состояние приложения в сервер
    start_server(app_state).await;
}

// Функция для подключения к базе данных
async fn check_db() -> Result<Arc<Mutex<AppState>>, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=test",
        NoTls
    ).await?;

    // Запускаем фоновую задачу для управления соединением
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Возвращаем Arc<Mutex<AppState>>
    Ok(Arc::new(Mutex::new(AppState { db_client: client })))
}

// Функция для старта сервера с переданным состоянием
async fn start_server(app_state: Arc<Mutex<AppState>>) {
    let app = create_routers(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


