# FROM rust:1.48.0
FROM rust:1.61.0

WORKDIR /app
COPY . /app

# RUN cargo install --path .

# RUN cargo build --release

# ENTRYPOINT ["/app/target/release/app"]
