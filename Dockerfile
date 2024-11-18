# Dockerfile
FROM rust:1.73-slim as builder

WORKDIR /usr/src/app

# Create blank project
RUN cargo init

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY src ./src

# Build application
RUN rm ./target/release/deps/student_api*
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/student-api .
COPY .env .

EXPOSE 3000

CMD ["./student-api"]