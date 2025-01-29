/// TypForge backend
///
/// This is an incredibly basic Axum backend to handle the LLM API and PDF rendering
/// for TypForge. I want the backend code (well really all of the code in this project)
/// to be as minimal as possible - so I'm just writing everything in the one file.
///
/// ju-lewis, 2025


use tokio;
use axum::{self, http::StatusCode, response::Html, routing::{get, post}, Router};
use tower_http::services::ServeDir;



#[tokio::main]
async fn main() {

    // Define app routes
    let app = Router::new()
        .route("/", get(read_index))
        .route("/template", post(create_template))
        .route("/rendered-pdf", get(render_pdf))
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

/// Query the Google Gemini?? API to generate an application-specific Typst cover letter template.
async fn create_template() {
    todo!();
}

/// Render the raw Typst code into a PDF
async fn render_pdf() {
    todo!();
}
