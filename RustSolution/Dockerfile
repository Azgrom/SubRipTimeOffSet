# 1. builder stage
#   - run unit tests
#   - build executable
#
FROM rust:slim-buster AS builder

WORKDIR /app
COPY . .

RUN cd deps/subrip_offset_api
RUN cargo test
RUN cd ../../
RUN cargo build --release

# 2. runtime stage
#   - start from clean debian container
#   - copy built executable
#
FROM debian:buster-slim AS runtime
WORKDIR /app

# app and configuration
COPY --from=builder /app/target/release/subrip_web_api subrip_web_api

EXPOSE 1111
ENTRYPOINT [ "./subrip_web_api" ]
