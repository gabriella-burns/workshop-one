To handle authentication and authorization in your `actix-web` API, you can use middleware to intercept requests and validate authentication tokens or credentials. Here's a basic example using JWT (JSON Web Tokens) for authentication.

### Plan

1. Add dependencies for JWT handling.
2. Create middleware to validate JWT tokens.
3. Protect specific routes using the middleware.

### Steps

1. **Add dependencies**: Update your [`Cargo.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22d8b9bd51-122c-4acf-9eaf-e5986be792eb%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/Cargo.toml") to include `jsonwebtoken` for JWT handling and `actix-web` for the web framework.

2. **Create middleware**: Define middleware to validate JWT tokens.

3. **Protect routes**: Apply the middleware to specific routes.

### Updated [`Cargo.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22d8b9bd51-122c-4acf-9eaf-e5986be792eb%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/Cargo.toml")

```toml
[dependencies]
actix-web = "4.0"
jsonwebtoken = "8.1"
serde = { version = "1.0", features = ["derive"] }
```

### Middleware and Route Protection

First, create a new file `auth.rs` for the authentication logic.

Next, update your `main.rs` to use the middleware and protect specific routes.

### Explanation

1. **Add dependencies**:
   - `jsonwebtoken` for handling JWT tokens.
   - `serde` for serializing and deserializing data.

2. **Create middleware**:
   - `Auth` middleware checks for the `Authorization` header, validates the JWT token, and extracts the claims.
   - If the token is valid, the request proceeds; otherwise, it returns a `401 Unauthorized` response.

3. **Protect routes**:
   - Apply the `Auth` middleware to the `App` instance using `.wrap(Auth)`.

### Final Steps

1. **Run the API**:
   ```sh
   cargo run
   ```

2. **Test the protected routes**:
   - Use a tool like `curl` to send requests with the `Authorization` header:
     ```sh
     curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://127.0.0.1:8080/hello
     ```
   - Replace `YOUR_JWT_TOKEN` with a valid JWT token. If the token is valid, you should receive the response from the protected route.
