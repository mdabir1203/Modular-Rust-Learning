FROM ubuntu:latest
LABEL authors="mabbas"

WORKDIR /src
COPY . .

RUN cargo install --path

CMD ["payment_gateway"]