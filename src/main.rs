
use tokio;
use axum::{self, http::StatusCode, response::Html, routing::get, Router};
use tower_http::services::ServeDir;



#[tokio::main]
async fn main() {

    // Define app routes
    let app = Router::new()
        .route("/", get(read_index))
        .fallback_service(ServeDir::new("frontend"));


    // Create TCP listener and serve app
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



/// Always returns the `index.html` file.
async fn read_index() -> Result<Html<String>, StatusCode> {

    let pwd = match std::env::current_dir() {
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(p) => p.to_string_lossy().into_owned()
    };


    match std::fs::read_to_string(pwd + "/frontend/index.html") {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(s) => Ok(Html(s))
    }
}


