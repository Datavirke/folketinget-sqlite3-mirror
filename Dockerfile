FROM clux/muslrust:latest AS builder

# Cache dependencies first using a dummy file.
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

# Add in our actual source code and build
ADD src ./src
ADD ./migrations/* ./migrations/
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /volume/target/x86_64-unknown-linux-musl/release/folketinget-sqlite3-mirror \
    /usr/local/bin/

EXPOSE 3030
VOLUME ["/data"]
ENV FTS_DATABASE_SQLITE_PATH=/data/folketinget.sqlite3
ENV FTS_SCRAPER_REQUESTS_PER_SECOND=5

CMD /usr/local/bin/folketinget-sqlite3-mirror