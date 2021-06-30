FROM rust:1.52 as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build
RUN cp target/release/cluster-agent /build/

FROM ubuntu:18.04

RUN ln -s /usr/local/lib64/libssl.so.1.1 /usr/lib64/libssl.so.1.1
RUN ln -s /usr/local/lib64/libcrypto.so.1.1 /usr/lib64/libcrypto.so.1.1

COPY --from=build /build/cluster-agent /

CMD ["/cluster-agent"]
