To handle query parameters in your API using `actix-web`, you can use the `web::Query` extractor. This allows you to define a struct that represents the query parameters and automatically parse them from the request.

### Plan

1. Define a struct to represent the query parameters.
2. Define a handler function that uses the `web::Query` extractor to parse the query parameters.
3. Register the handler function with the `App` instance.

### Example

Let's create a new endpoint `/search` that accepts query parameters `q` (query) and `page`.

### Steps

1. Define a struct to represent the query parameters.
2. Define a handler function that uses the `web::Query` extractor.
3. Register the handler function with the `App` instance.

### Explanation

1. **Define a struct to represent the query parameters:**
   - `SearchQuery` struct with fields `q` (query) and `page` (optional).

2. **Define the new handler function:**
   - `search`: Uses the `web::Query` extractor to parse the query parameters and returns a formatted string with the query and page number.

3. **Register the new handler function:**
   - Add `.service(search)` to the `App` instance in the `main` function.

### Final Steps

1. **Run the API:**
   ```sh
   cargo run
   ```

2. **Test the new endpoint:**
   - Use a web browser or a tool like `curl` to send a GET request to `http://127.0.0.1:8080/search?q=example&page=2`:
     ```sh
     curl "http://127.0.0.1:8080/search?q=example&page=2"
     ```
   - You should receive a response with the search query and page number:
     ```plaintext
     Search query: example, Page: 2
     ```
