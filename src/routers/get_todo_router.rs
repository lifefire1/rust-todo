use axum::{
    routing::get,
    Router,
};

include!("../handlers/get_todo_handler.rs");


fn create_routers() -> Router {
    Router::new()
        .route("/todo", get(handle_get_request))
        .route("/todo/{todoid}", get(handle_get_request))
}