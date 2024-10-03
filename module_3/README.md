To handle POST requests in your API using `actix-web`, you need to define a handler function that processes the incoming data and register it with the `App` instance. Here's how you can do it:

### Plan

1. Define a new handler function for the POST request.
2. Register the new handler function with the `App` instance.
3. Use the `web::Json` extractor to handle JSON payloads.

### Example

Let's create a new endpoint `/echo` that accepts a JSON payload and echoes it back.

### Steps

1. Define a new handler function for the POST request.
2. Register the new handler function with the `App` instance.

### Explanation

1. **Define a struct to represent the JSON payload:**
   - `EchoRequest` struct with a single field `message`.

2. **Define the new handler function:**
   - `echo`: Takes a JSON payload, extracts the `message` field, and echoes it back as a JSON response.

3. **Register the new handler function:**
   - Add `.service(echo)` to the `App` instance in the `main` function.

### Final Steps

1. **Run the API:**
   ```sh
   cargo run
   ```

2. **Test the new POST endpoint:**
   - Use a tool like `curl` or Postman to send a POST request to `http://127.0.0.1:8080/echo` with a JSON payload:
     ```sh
     curl -X POST -H "Content-Type: application/json" -d '{"message": "Hello, API!"}' http://127.0.0.1:8080/echo
     ```
   - You should receive a response with the echoed message:
     ```json
     {
       "message": "Hello, API!"
     }
     ```
