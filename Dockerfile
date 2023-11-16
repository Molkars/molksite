FROM alpine:latest

COPY . /build
WORKDIR /build

RUN apk add curl gcc musl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source $HOME/.cargo/env; \
    cargo build --target x86_64-unknown-linux-musl
RUN mv target/x86_64-unknown-linux-musl/debug /app
RUN rm -rf /build

WORKDIR /app
CMD ["/bin/sh"]