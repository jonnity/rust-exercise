# latest rust image 
FROM rust:1.85-bookworm
RUN rustup component add rustfmt clippy
