/// TypForge backend
///
/// This is an incredibly basic Axum backend to handle the LLM API and PDF rendering
/// for TypForge. I want the backend code (well really all of the code in this project)
/// to be as minimal as possible - so I'm just writing everything in the one file.
///
/// ju-lewis, 2025


use tokio;
use axum::{self, extract::{Json, State}, http::{header, HeaderMap, StatusCode}, response::{Html, IntoResponse}, routing::{get, post}, Router};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;
use typst::{foundations::Bytes, text::Font};
use typst_as_lib;


const GEMINI_API_ENDPOINT: &'static str  = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key=";
const API_KEY: &'static str = env!("GEMINI_API_KEY");
const FONT: &[u8] = include_bytes!("../IBMPlexSerif-Regular.ttf");


#[derive(Clone)]
struct AppState {
    client: Client
}

#[derive(Deserialize, Serialize)]
struct TemplateRequest {
    cv: String,
    spec: String
}

#[derive(Deserialize)]
struct RenderRequest {
    code: String
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
        .route("/render-pdf", post(render_pdf))
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
    let prompt = match std::fs::read_to_string("alt_prompt.txt") {
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
        eprintln!("Error accessing AI API!: {:?}", res.unwrap_err());
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let json: Value = match res.unwrap().json().await {
        Err(e) => {
            eprintln!("Error getting response JSON: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(j) => j
    };

    // Pull text from API response
    let content = match json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        None => {
            eprintln!("Error getting content from JSON");
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Some(c) => c
    }.to_string();

    Ok(content)
}

/// Render the raw Typst code into a PDF
async fn render_pdf(Json(r): Json<RenderRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {

    let typst_source = r.code;

    
    let font = match Font::new(Bytes::from(FONT), 0) {
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Couldn't load font".to_string())),
        Some(f) => f
    };
    let template = typst_as_lib::TypstTemplate::new(vec![font], typst_source);

    let document = match template.compile_with_input(typst::foundations::Dict::new()).output {
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Compilation failed".to_string())),
        Ok(d) => d
    };

    let export_options = Default::default();
    let pdf = match typst_pdf::pdf(&document, &export_options) {
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Rendering PDF failed".to_string())),
        Ok(p) => p
    };

    // Save to file
    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Somehow my server's system time broke lmao".to_string())),
        Ok(n) => n.as_millis()
    };

    let write_result = std::fs::write(format!("pdf/{}.pdf", now), &pdf);

    
    if write_result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "There was an error caching your letter.".to_string()));
    }

    // Create headers for an inline file
    //let mut headers = HeaderMap::new();
    //headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/pdf"));
    //headers.insert(header::CONTENT_DISPOSITION, header::HeaderValue::from_static("inline"));
    // Return headers and PDF contents
    //Ok((headers, axum::body::Bytes::from(pdf)))

    // Return filename
    Ok(format!("{:?}", now))
}





