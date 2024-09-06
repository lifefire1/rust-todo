use log::{info};
use chrono::Local;
include!("routers/get_todo_router.rs");


#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    info!("{:20} {}","application running", Local::now());
    start_server().await;
}

async fn start_server(){
    let app = create_routers();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
