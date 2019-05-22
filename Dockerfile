FROM rust

COPY . /brust

WORKDIR /brust

RUN cargo build --release

EXPOSE 8787

VOLUME /brust/config

CMD cargo run --release
