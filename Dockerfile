FROM rust:1.53 as builder
WORKDIR /usr/src/dezrevellou

COPY . .
RUN cargo install --path .

FROM debian:buster-slim
MAINTAINER Paul Ollivier <contact@paulollivier.fr>

ENV RUST_LOG=debug
RUN apt update && apt install -y libc6 && rm -rf /var/lib/apt/lists/
COPY --from=builder /usr/local/cargo/bin/dezrevellou /dezrevellou
RUN chmod +x /dezrevellou
CMD ["/dezrevellou"]