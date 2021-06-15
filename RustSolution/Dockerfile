# 1. builder stage
#   - run unit tests
#   - build executable
#
FROM rust:slim-buster AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# 2. runtime stage
#   - start from clean debian container
#   - copy built executable
#
FROM debian:buster-slim AS runtime
WORKDIR /app

# app and configuration
COPY --from=builder /app/target/release/subrip_web_api subrip_web_api
ENTRYPOINT [ "./subrip_web_api" ]
