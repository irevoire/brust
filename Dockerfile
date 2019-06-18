FROM rust:stretch

RUN git clone https://github.com/irevoire/brust && \
	cd brust && \
	cargo build --release && \
	mv target/release/brust . && \
	rm -rf src target

WORKDIR brust

VOLUME /brust/config

CMD ./brust
