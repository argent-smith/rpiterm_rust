FROM rust:1.36-slim as build

RUN cd / && USER=root cargo new --bin rpiterm_rust
WORKDIR /rpiterm_rust

COPY Cargo.lock Cargo.toml /rpiterm_rust/

COPY ./src ./src

RUN cargo build --release

FROM debian:stretch-slim

COPY --from=build /rpiterm_rust/target/release/rpiterm_rust /bin

RUN addgroup rpiterm
RUN useradd -g rpiterm rpiterm

USER rpiterm
WORKDIR /home/rpiterm
CMD ["/bin/rpiterm_rust", "-p", "9200"]
EXPOSE 9200
