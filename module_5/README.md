To handle request headers in your API using `actix-web`, you can use the `HttpRequest` type to access the headers. Here's how you can do it:

### Plan

1. Define a handler function that takes an `HttpRequest` parameter.
2. Extract and process the headers from the `HttpRequest`.
3. Register the handler function with the `App` instance.

### Example

Let's create a new endpoint `/headers` that reads a custom header `X-Custom-Header` and returns its value.

### Steps

1. Define the handler function that processes the headers.
2. Register the handler function with the `App` instance.

### Explanation

1. **Define the new handler function:**
   - `headers`: Takes an `HttpRequest` parameter, checks for the presence of the `X-Custom-Header`, and returns its value if found. If the header is not found or is invalid, it returns a `BadRequest` response.

2. **Register the new handler function:**
   - Add `.service(headers)` to the `App` instance in the `main` function.

### Final Steps

1. **Run the API:**
   ```sh
   cargo run
   ```

2. **Test the new endpoint:**
   - Use a tool like `curl` to send a GET request to `http://127.0.0.1:8080/headers` with the custom header:
     ```sh
     curl -H "X-Custom-Header: MyHeaderValue" http://127.0.0.1:8080/headers
     ```
   - You should receive a response with the header value:
     ```plaintext
     X-Custom-Header: MyHeaderValue
     ```
