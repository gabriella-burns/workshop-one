// How can I handle error responses in my API?
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fmt;

// Define custom error types
#[derive(Debug, Error)]
enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Internal server error")]
    InternalServerError,
}

// Implement ResponseError for MyError
impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::InvalidInput(msg) => {
                HttpResponse::BadRequest().body(format!("Invalid input: {}", msg))
            }
            MyError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal server error")
            }
        }
    }
}

// Existing endpoint
#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

// New endpoint: /goodbye
#[get("/goodbye")]
async fn goodbye() -> impl Responder {
    "Goodbye, world!"
}

// New endpoint: /greet/{name}
#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> Result<String, MyError> {
    if name.is_empty() {
        return Err(MyError::InvalidInput("Name cannot be empty".into()));
    }
    Ok(format!("Hello, {}!", name))
}

// Define a struct to represent the JSON payload
#[derive(Deserialize, Serialize)]
struct EchoRequest {
    message: String,
}

// New endpoint: /echo
#[post("/echo")]
async fn echo(req: web::Json<EchoRequest>) -> Result<HttpResponse, MyError> {
    if req.message.is_empty() {
        return Err(MyError::InvalidInput("Message cannot be empty".into()));
    }
    Ok(HttpResponse::Ok().json(req.into_inner()))
}

// Define a struct to represent the query parameters
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    page: Option<u32>,
}

// New endpoint: /search
#[get("/search")]
async fn search(query: web::Query<SearchQuery>) -> Result<String, MyError> {
    if query.q.is_empty() {
        return Err(MyError::InvalidInput("Query cannot be empty".into()));
    }
    let page = query.page.unwrap_or(1);
    Ok(format!("Search query: {}, Page: {}", query.q, page))
}

// Define a struct to represent the request body
#[derive(Deserialize)]
struct SubmitRequest {
    name: String,
    age: u32,
}

// New endpoint: /submit
#[post("/submit")]
async fn submit(req: web::Json<SubmitRequest>) -> Result<String, MyError> {
    if req.name.is_empty() {
        return Err(MyError::InvalidInput("Name cannot be empty".into()));
    }
    if req.age == 0 {
        return Err(MyError::InvalidInput("Age must be greater than 0".into()));
    }
    Ok(format!("Received: name = {}, age = {}", req.name, req.age))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(goodbye)
            .service(greet)
            .service(echo)
            .service(search)
            .service(submit)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
