# HTTP Server in Rust

A minimal, high-performance HTTP server built from scratch in Rust with no external dependencies. This project demonstrates the fundamental principles of an HTTP server by directly handling TCP connections and raw HTTP requests.

The server listens on port `:8080` and responds with "Hello, World!" to GET `/` requests.

## 🚀 Why Rust?

-   **Performance**: Rust provides C-level performance, enabling sub-millisecond latency and high throughput
-   **Reliability**: Rust's ownership model guarantees memory and thread safety, preventing entire classes of common bugs in systems programming
-   **Low-level Control**: The standard library offers powerful, low-level APIs for networking, allowing us to build a server from the ground up without external packages

## 🐳 How to Run with Docker

This is the recommended way to run the server. You must have Docker installed.

### Build the Docker Image

Open your terminal in the project directory and run:

```bash
docker build -t http-server .
```

### Run the Docker Container

Once the image is built, run it with the following command. This maps port 8080 on your local machine to port 8080 in the container:

```bash
docker run --rm -p 8080:8080 http-server
```

You should see the output:

```
Server listening on port 8080...
```

## 🧪 How to Test

Once the server is running, you can test it using a tool like curl or by visiting it in your web browser.

### Using curl

Open a new terminal window and run:

```bash
curl http://localhost:8080
```

You should receive the response:

```
Hello, World!
```

### Using a Web Browser

Navigate to [http://localhost:8080](http://localhost:8080) in your web browser. The page will display "Hello, World!".

## 🏗️ How to Run Locally (Development)

If you want to run the server directly without Docker:

### Prerequisites

-   [Rust](https://rustup.rs/) installed on your system

### Build and Run

```bash
# Build the project
cargo build --release

# Run the server
cargo run
```

The server will start and listen on port 8080.

## 📋 Features

-   ✅ **Zero Dependencies**: Built using only Rust standard library
-   ✅ **Multi-threaded**: Handles concurrent connections using threads
-   ✅ **HTTP/1.1 Compliant**: Proper HTTP response formatting
-   ✅ **Error Handling**: Graceful error handling for network issues
-   ✅ **Docker Support**: Containerized for easy deployment
-   ✅ **Minimal Image Size**: Docker image is only ~10MB

## 🔧 API Endpoints

| Method | Path | Response        | Status Code   |
| ------ | ---- | --------------- | ------------- |
| GET    | `/`  | "Hello, World!" | 200 OK        |
| GET    | `/*` | "Not Found"     | 404 NOT FOUND |

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
