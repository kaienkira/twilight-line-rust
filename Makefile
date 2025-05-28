.PHONY: \
default \
build-debug \
build-release \
build-windows \
clean \
test \
fmt

default: build-release

build-debug:
	@cargo build

build-release:
	@cargo build --release

build-windows:
	@cargo build --release --target x86_64-pc-windows-gnu

clean:
	@cargo clean

test:
	@cargo test -- --nocapture

fmt:
	@cargo fmt
