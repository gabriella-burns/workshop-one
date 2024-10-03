To handle error responses in your `actix-web` API, you can define custom error types and implement the `ResponseError` trait for them. This allows you to return structured error responses with appropriate HTTP status codes and messages.

### Plan

1. Define custom error types.
2. Implement the `ResponseError` trait for the custom error types.
3. Use the custom error types in your handler functions.

### Example

Let's create a custom error type and use it in a handler function.

### Steps

1. Define a custom error type.
2. Implement the `ResponseError` trait for the custom error type.
3. Use the custom error type in a handler function.

### Updated Code

First, add the `thiserror` crate to your [`Cargo.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22d8b9bd51-122c-4acf-9eaf-e5986be792eb%22%5D "/Users/burnsg/Library/CloudStorage/OneDrive-moodys.com/Desktop/WIR/workshop_one/Cargo.toml") file:

```toml
[dependencies]
actix-web = "4.0"
thiserror = "1.0"
```

Next, update your `main.rs` file to include custom error handling.


### Explanation

1. **Add dependencies**:
   - `thiserror` for defining custom error types.

2. **Define custom error types**:
   - `MyError` enum with variants for different error types.

3. **Implement `ResponseError` for custom error types**:
   - Implement the `ResponseError` trait for `MyError` to define how each error type should be converted to an HTTP response.

4. **Use custom error types in handler functions**:
   - Update handler functions to return `Result<T, MyError>` and use the custom error types for error handling.

### Final Steps

1. **Run the API**:
   ```sh
   cargo run
   ```

2. **Test the error handling**:
   - Use a tool like `curl` to send requests that trigger errors and observe the structured error responses:
     ```sh
     curl "http://127.0.0.1:8080/greet/"
     ```
   - You should receive a `400 Bad Request` response with the message "Invalid input: Name cannot be empty".
