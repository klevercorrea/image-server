FROM rust:1.73.0-alpine as builder
RUN apk add --no-cache musl-dev sqlite-static openssl-dev openssl-libs-static pkgconf git libpq-dev

# Set `SYSROOT` to a dummy path (default is /usr) because pkg-config-rs *always*
# links those located in that path dynamically but we want static linking, c.f.
# https://github.com/rust-lang/pkg-config-rs/blob/54325785816695df031cef3b26b6a9a203bbc01b/src/lib.rs#L613
ENV SYSROOT=/dummy

# The env vars tell libsqlite3-sys to statically link libsqlite3.
ENV SQLITE3_STATIC=1 SQLITE3_LIB_DIR=/usr/lib/

# The env var tells pkg-config-rs to statically link libpq.
ENV LIBPQ_STATIC=1

WORKDIR /app
COPY . /app
RUN cargo build --bins --release

FROM scratch
ARG version=unknown
ARG release=unreleased
LABEL name="Image Server" \
      maintainer="klever.silveira@codex.com.br" \
      vendor="Codex" \
      version=${version} \
      release=${release} \
      summary="A high-performance image server for use as a content delivery network (CDN)" \
      description="An image server to use as CDN"

# COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/target/release/image-server /image-server
EXPOSE 8080
CMD ["/image-server"]