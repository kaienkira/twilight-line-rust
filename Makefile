.PHONY: \
default \
build-release \
build_debug \
clean \
run-client \
run-server

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
