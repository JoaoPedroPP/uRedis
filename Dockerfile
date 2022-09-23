FROM ekidd/rust-musl-builder:stable as cargo-build

WORKDIR /usr/src/app

COPY . .

RUN sudo chown -R rust:rust /usr/src/app && cargo build --release

FROM alpine:latest

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY certi.pem /usr/local/share/ca-certificates/certi.crt

RUN update-ca-certificates

WORKDIR /src/app

COPY .env .

COPY --from=cargo-build /usr/src/app/target/x86_64-unknown-linux-musl/release/uservice-redis .

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Run the application
CMD ["./uservice-redis"]