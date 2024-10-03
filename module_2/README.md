To add additional endpoints to your basic API, you need to define new handler functions and register them with the [`App`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A0%2C%22character%22%3A21%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") instance in your [`main`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A7%2C%22character%22%3A13%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") function. Here's how you can do it:

### Plan

1. Define new handler functions for the additional endpoints.
2. Register these new handler functions with the [`App`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A0%2C%22character%22%3A21%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") instance in the [`main`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A7%2C%22character%22%3A13%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") function.

### Example

Let's add two new endpoints:
1. `/goodbye` that returns "Goodbye, world!"
2. `/greet/{name}` that returns a personalized greeting.

### Steps

1. Define the new handler functions.
2. Register the new handler functions with the [`App`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A0%2C%22character%22%3A21%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") instance.

### Explanation

1. **Define new handler functions:**
   - `goodbye`: Returns "Goodbye, world!".
   - `greet`: Takes a path parameter `name` and returns a personalized greeting.

2. **Register the new handler functions:**
   - Add `.service(goodbye)` and `.service(greet)` to the [`App`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A0%2C%22character%22%3A21%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") instance in the [`main`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fburnsg%2FLibrary%2FCloudStorage%2FOneDrive-moodys.com%2FDesktop%2FWIR%2Fworkshop_one%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A7%2C%22character%22%3A13%7D%7D%5D%2C%22fba7dbae-76fa-40f2-bdf3-029bfea4d1e4%22%5D "Go to definition") function.

### Final Steps

1. **Run the API:**
   ```sh
   cargo run
   ```

2. **Access the new endpoints:**
   - Open a web browser and navigate to `http://127.0.0.1:8080/goodbye` to see "Goodbye, world!".
   - Navigate to `http://127.0.0.1:8080/greet/YourName` to see "Hello, YourName!".
