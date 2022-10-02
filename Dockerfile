FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /bvh
WORKDIR /bvh/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /bvh/fuzz/target/x86_64-unknown-linux-gnu/release/bvh-fuzz /