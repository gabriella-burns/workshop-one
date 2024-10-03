// How can I handle POST requests in my API?

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

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
#[derive(Deserialize, Serialize)]
struct EchoRequest {
    message: String,
}

// New endpoint: /echo
#[post("/echo")]
async fn echo(req: web::Json<EchoRequest>) -> impl Responder {
    HttpResponse::Ok().json(req.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(goodbye)
            .service(greet)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
