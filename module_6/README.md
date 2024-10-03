To handle the request body in your API using `actix-web`, you can use the `web::Json` extractor for JSON payloads or `web::Form` for form data. Here's how you can do it:

### Plan

1. Define a struct to represent the request body.
2. Define a handler function that uses the appropriate extractor to parse the request body.
3. Register the handler function with the `App` instance.

### Example

Let's create a new endpoint `/submit` that accepts a JSON payload with fields `name` and `age`.

### Steps

1. Define a struct to represent the request body.
2. Define a handler function that uses the `web::Json` extractor.
3. Register the handler function with the `App` instance.

### Explanation

1. **Define a struct to represent the request body:**
   - `SubmitRequest` struct with fields `name` and `age`.

2. **Define the new handler function:**
   - `submit`: Uses the `web::Json` extractor to parse the JSON payload and returns a formatted string with the received data.

3. **Register the new handler function:**
   - Add `.service(submit)` to the `App` instance in the `main` function.

### Final Steps

1. **Run the API:**
   ```sh
   cargo run
   ```

2. **Test the new endpoint:**
   - Use a tool like `curl` to send a POST request to `http://127.0.0.1:8080/submit` with a JSON payload:
     ```sh
     curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe", "age": 30}' http://127.0.0.1:8080/submit
     ```
   - You should receive a response with the received data:
     ```plaintext
     Received: name = John Doe, age = 30
     ```
