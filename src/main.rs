mod api;
mod cli;
mod errors;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest};
use errors::ApiError;
use models::NcbiResponse;
use std::error::Error;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use env_logger::Env;
use lazy_static::lazy_static;

// Global state for rate limiting
lazy_static! {
    static ref RATE_LIMIT_STORE: Mutex<HashMap<String, (SystemTime, u32)>> = Mutex::new(HashMap::new());
}

// Rate limiting middleware
async fn rate_limiter(req: &HttpRequest) -> Result<(), ApiError> {
    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let mut store = RATE_LIMIT_STORE.lock().unwrap();

    let (last_request_time, request_count) = store.entry(ip).or_insert((SystemTime::now(), 0));

    let elapsed = last_request_time.elapsed().unwrap_or(Duration::from_secs(0));

    if elapsed > Duration::from_secs(60) {
        // Reset the counter if more than 60 seconds have passed
        *last_request_time = SystemTime::now();
        *request_count = 1;
    } else if *request_count >= 10 {
        // Reject the request if the limit is exceeded
        return Err(ApiError::RateLimitExceeded);
    } else {
        // Increment the request count
        *request_count += 1;
    }

    Ok(())
}

// Handler for searching by genus
async fn search_by_genus_handler(req: HttpRequest, query: web::Path<String>) -> Result<HttpResponse, ApiError> {
    rate_limiter(&req).await?;  // Apply rate limiting
    let ids = search_by_genus(&query).await.map_err(|e| ApiError::NcbiApiError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(ids))
}

// Handler for searching by author
async fn search_by_author_handler(req: HttpRequest, query: web::Path<String>) -> Result<HttpResponse, ApiError> {
    rate_limiter(&req).await?;  // Apply rate limiting
    let ids = search_by_author(&query).await.map_err(|e| ApiError::NcbiApiError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(ids))
}

// Handler for fetching details by ID
async fn fetch_details_handler(req: HttpRequest, id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    rate_limiter(&req).await?;  // Apply rate limiting
    let details = fetch_details(&id).await.map_err(|e| ApiError::NcbiApiError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(details))
}

// Function to search by genus/species
async fn search_by_genus(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let esearch_url = api::build_esearch_url(query, "Organism");
    let esearch_body = api::fetch_api_response(&esearch_url).await?;
    let ncbi_response: NcbiResponse = serde_json::from_str(&esearch_body)?;
    Ok(ncbi_response.esearchresult.idlist)
}

// Function to search by author name
async fn search_by_author(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let esearch_url = api::build_esearch_url(query, "Author");
    let esearch_body = api::fetch_api_response(&esearch_url).await?;
    let ncbi_response: NcbiResponse = serde_json::from_str(&esearch_body)?;
    Ok(ncbi_response.esearchresult.idlist)
}

// Function to fetch details for a specific ID
async fn fetch_details(id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    // Fetch GenBank format data
    let genbank_url = api::build_efetch_url(id, "gb");
    let genbank_body = api::fetch_api_response(&genbank_url).await?;

    // Fetch FASTA format data
    let fasta_url = api::build_efetch_url(id, "fasta");
    let fasta_body = api::fetch_api_response(&fasta_url).await?;

    // Combine responses into a single JSON object
    let response = serde_json::json!({
        "genbank": genbank_body,
        "fasta": fasta_body,
    });
    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Start the Actix Web server
    HttpServer::new(|| {
        App::new()
            .route("/search/genus/{query}", web::get().to(search_by_genus_handler))
            .route("/search/author/{query}", web::get().to(search_by_author_handler))
            .route("/details/{id}", web::get().to(fetch_details_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}