FROM rust:1.57-bullseye as builder

COPY . .
RUN cargo install --path .

FROM debian:bullseye 
RUN apt update && apt install -y ca-certificates && apt clean
COPY --from=builder /usr/local/cargo/bin/dnscheck /usr/local/bin/dnscheck
ENTRYPOINT ["dnscheck"]

