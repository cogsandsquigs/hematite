# Latest rust version, using a slim image
# Can't use alpine because of musl
FROM rust:1.66-slim

# Copy over the source code
COPY . .

# Expose the port
EXPOSE 8080

# Install the dependencies
RUN cargo build --release

# Move the binary to the root
RUN mv target/release/hematite .

# Delete the build files
RUN rm -rf target

CMD ["hematite"]
