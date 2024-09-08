use axum::{
    routing::get,
    Router,
};

include!("../handlers/get_todo_handler.rs");
include!("../handlers/post_todo.rs");


fn create_routers(app_state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/todo", get(handle_get_request).post(post_todo))
        .route("/todo/{todoid}", get(handle_get_request))
        .layer(Extension(app_state))
}