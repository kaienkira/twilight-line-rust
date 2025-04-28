.PHONY: \
default \
build-debug \
build-release \
clean \
run-client \
run-server \
test

default: build-release

build-debug:
	cargo build

build-release:
	cargo build --release

clean:
	cargo clean

run-client:
	cargo run --bin twilight-line-rust-client

run-server:
	cargo run --bin twilight-line-rust-server

test:
	cargo test -- --nocapture
