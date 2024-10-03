// How can I handle request headers in my API?

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use serde::Deserialize;

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
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

// Define a struct to represent the JSON payload
#[derive(Deserialize)]
struct EchoRequest {
    message: String,
}

// New endpoint: /echo
#[post("/echo")]
async fn echo(req: web::Json<EchoRequest>) -> impl Responder {
    HttpResponse::Ok().json(req.into_inner())
}

// Define a struct to represent the query parameters
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    page: Option<u32>,
}

// New endpoint: /search
#[get("/search")]
async fn search(query: web::Query<SearchQuery>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    format!("Search query: {}, Page: {}", query.q, page)
}

// New endpoint: /headers
#[get("/headers")]
async fn headers(req: HttpRequest) -> impl Responder {
    if let Some(header_value) = req.headers().get("X-Custom-Header") {
        if let Ok(header_str) = header_value.to_str() {
            return HttpResponse::Ok().body(format!("X-Custom-Header: {}", header_str));
        }
    }
    HttpResponse::BadRequest().body("X-Custom-Header not found or invalid")
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
            .service(headers)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
