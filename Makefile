.PHONY: \
default \
build-debug \
build-release \
clean \
test \
fmt

default: build-release

build-debug:
	@cargo build

build-release:
	@cargo build --release

clean:
	@cargo clean

test:
	@cargo test -- --nocapture

fmt:
	@cargo fmt
