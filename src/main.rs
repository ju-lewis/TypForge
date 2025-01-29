/// TypForge backend
///
/// This is an incredibly basic Axum backend to handle the LLM API and PDF rendering
/// for TypForge. I want the backend code (well really all of the code in this project)
/// to be as minimal as possible - so I'm just writing everything in the one file.
///
/// ju-lewis, 2025


use tokio;
use axum::{self, extract::{State, Json}, http::StatusCode, response::Html, routing::{get, post}, Router};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;



const GEMINI_API_ENDPOINT: &'static str  = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=$";
const API_KEY: &'static str = env!("GEMINI_API_KEY");


#[derive(Clone)]
struct AppState {
    client: Client
}

#[derive(Deserialize, Serialize)]
struct TemplateRequest {
    cv: String,
    spec: String
}



#[tokio::main]
async fn main() {

    let state = AppState {
        client: Client::new()
    };

    // Define app routes
    let app = Router::new()
        .route("/", get(read_index))
        .route("/template", post(create_template))
        .route("/rendered-pdf", get(render_pdf))
        .fallback_service(ServeDir::new("frontend"))
        .with_state(state);


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
async fn create_template(State(state): State<AppState>, Json(r): Json<TemplateRequest>) -> Result<String, StatusCode> {

    let url = String::from(GEMINI_API_ENDPOINT) + API_KEY;
    
    // Create full prompt
    let prompt = match std::fs::read_to_string("prompt.txt") {
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(p) => p + &format!("\nCV Content: {}\n\nJob/application listing content: {}", r.cv, r.spec)
    };


    let res = state.client.post(url)
        .header("Content-Type", "application/json")
        .body(json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        }).to_string())
        .send().await;
    
    if res.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let json: Value = match res.unwrap().json().await {
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(j) => j
    };

    // Pull text from API response
    let content = match json["candidates"][0]["content"].as_str() {
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Some(c) => c
    }.to_string();

    

    Ok(content)
}

/// Render the raw Typst code into a PDF
async fn render_pdf() {
    todo!();
}
