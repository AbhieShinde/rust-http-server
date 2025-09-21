# Stage 1: Build the application
# We use the official Rust alpine image.
FROM rust:1-alpine AS builder

# Set the working directory inside the container.
WORKDIR /usr/src/app

# Copy the project files into the container.
COPY Cargo.toml .
COPY main.rs .

# Build the application in release mode for optimal performance.
RUN cargo build --release

# Stage 2: Create the final, minimal image
# Use the scratch image, which is an empty, minimal base.
FROM scratch

# Copy the compiled binary from the builder stage to the final image.
COPY --from=builder /usr/src/app/target/release/http-server /http-server

# Expose port 8080 to allow external traffic to the server.
EXPOSE 8080

# Set the command to run when the container starts.
CMD [ "/http-server" ]
