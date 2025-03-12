# Dependencies

- **actix-web (4.9.0)** - An asynchronous web framework for Rust that allows you to build high-performance web applications. Used to create HTTP servers, handle routing, requests, and responses.
- **mongodb (3.2.2)** - The official MongoDB driver for Rust that enables your application to connect and interact with MongoDB databases.
- **serde (1.0)** - A serialization/deserialization framework for Rust. Allows data conversion between Rust formats and formats like JSON. The "derive" feature enables the use of the `#[derive(Serialize, Deserialize)]` macro.
- **serde_json (1.0)** - JSON implementation for serde, enabling parsing and creation of JSON data.
- **dotenv (0.15.0)** - A library for loading environment variables from a `.env` file, useful for application configuration.
- **futures (0.3.31)** - A library for asynchronous programming in Rust, providing data types and functions for working with asynchronous operations.
- **tokio (1.34.0)** - A high-performance asynchronous runtime for Rust. The "full" feature enables all tokio functionality like I/O, scheduling, and macros.
- **bson (2.13.0)** - A library for working with BSON (Binary JSON) format, which is used by MongoDB. The "chrono" feature adds support for date and time types.
- **thiserror (2.0.12)** - A library for creating custom error types in Rust more easily through derive macros.
- **env_logger (0.10.1)** - A logging library that configures itself from environment variables, suitable for use with the `log` library.
- **log (0.4)** - A logging facade for Rust that provides a configurable logging API.

```toml
[dependencies]
actix-web = "4.9.0"
mongodb = "3.2.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15.0"
futures = "0.3.31"
tokio = { version = "1.34.0", features = ["full"] }
bson = { version = "2.13.0", features = ["chrono"] }
thiserror = "2.0.12"
env_logger = "0.10.1"
log = "0.4"
```

# How to Run the Application

1. **Install Rust and Cargo**
    - Make sure you have Rust and Cargo installed. If not, visit [rustup.rs](https://rustup.rs) to install them.

2. **Set Up MongoDB**
    - Install MongoDB on your system or use a MongoDB Atlas cloud instance
    - Make sure MongoDB is running on your local machine or you have the connection string for your cloud instance

3. **Configure Environment Variables**
    - Create a `.env` file in the root directory of your project
    - Add the following variables (modify as needed):
      ```
      MONGODB_URI=mongodb://localhost:27017
      DATABASE_NAME=your_database_name
      PORT=3000
      ```

4. **Build and Run the Project**
    - Open terminal in the project directory
    - Run the application:
      ```
      cargo run
      ```
    - For a release build:
      ```
      cargo run --release
      ```

5. **Testing the API**
    - Once running, you can access the API at `http://localhost:3000` (or whatever port you specified)
    - Use tools like Postman, curl, or your browser to test the endpoints

6. **Development**
    - For development with auto-reload, you can use:
      ```
      cargo watch -x run
      ```
    - Install with: `cargo install cargo-watch`

7. **Logging**
    - Set the log level using environment variables:
      ```
      RUST_LOG=info cargo run
      ```
    - Available levels: error, warn, info, debug, trace

