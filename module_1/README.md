### Plan

1. Create a new Rust project using Cargo.
2. Add dependencies for creating an API (e.g., `actix-web`).
3. Create a basic API with a single endpoint that returns a "Hello, world!" message.

### Steps

1. Open the terminal in VS Code.
2. Run the command to create a new Rust project.
3. Add `actix-web` to the [`Cargo.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%222eaa6d45-9b6b-43d6-9be4-c3268eb4dc0a%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/Cargo.toml") file.
4. Modify the [`main.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%222eaa6d45-9b6b-43d6-9be4-c3268eb4dc0a%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/src/main.rs") file to set up a basic API.

### Commands and Code

1. **Create a new Rust project:**
   ```sh
   cargo new basic_api
   cd basic_api
   ```

2. **Add dependencies to [`Cargo.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%222eaa6d45-9b6b-43d6-9be4-c3268eb4dc0a%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/Cargo.toml"):**
   ```toml
   [dependencies]
   actix-web = "4.0"
   ```

3. **Modify [`main.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%222eaa6d45-9b6b-43d6-9be4-c3268eb4dc0a%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/src/main.rs") to create a basic API:**

   ```rust
   use actix_web::{get, App, HttpServer, Responder};

   #[get("/")]
   async fn hello() -> impl Responder {
       "Hello, world!"
   }

   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
       HttpServer::new(|| {
           App::new()
               .service(hello)
       })
       .bind("127.0.0.1:8080")?
       .run()
       .await
   }
   ```

### Final Steps

1. **Run the API:**
   ```sh
   cargo run
   ```

2. **Access the API:**
   Open a web browser and navigate to `http://127.0.0.1:8080` to see the "Hello, world!" message.
