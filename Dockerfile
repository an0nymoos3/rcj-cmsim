# Use the official rust image, same version as I've been building with.
FROM rust:1.76 AS builder

# Set working directory
#WORKDIR ../

# Copy src dir to docker dir
COPY ./ ./

# Compile source code
RUN cargo build --release

#CMD ["./target/release/rcj-cmsim"]
