// How can I handle authentication and authorization in my API?
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
mod auth;
use auth::Auth;

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

// Define a struct to represent the request body
#[derive(Deserialize)]
struct SubmitRequest {
    name: String,
    age: u32,
}

// New endpoint: /submit
#[post("/submit")]
async fn submit(req: web::Json<SubmitRequest>) -> impl Responder {
    let data = req.into_inner();
    format!("Received: name = {}, age = {}", data.name, data.age)
}

// New endpoint: /upload
#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.try_next().await? {
        let mut field = item;
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap();

        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
    }
    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Auth) // Apply the Auth middleware
            .service(hello)
            .service(goodbye)
            .service(greet)
            .service(echo)
            .service(search)
            .service(submit)
            .service(upload)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
