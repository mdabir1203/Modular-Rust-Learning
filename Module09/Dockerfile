FROM rust::1.68 as builder

WORKDIR /usr/src/backend
COPY . .

RUN cargo build --release

FROM debian::buster-slim

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

CMD ["backend"]