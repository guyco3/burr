.PHONY: all build build-guests build-ex build-int test test-unit test-int clean

# Ensure burr is built in release mode and in path
BURR_BIN := $(PWD)/target/release/burr

all: build build-guests build-ex build-int

# Build the main burr CLI
build:
	@echo "=== Building burr CLI ==="
	cargo build -p burr --release

# Build all WASM guest components
build-guests:
	@echo "=== Building Supply Chain Guest Apps ==="
	cd tests/integration/scenarios/telemetry-exfiltration/guest && cargo build --target=wasm32-wasip2 --release
	cd tests/integration/scenarios/credential-harvester/guest && cargo build --target=wasm32-wasip2 --release
	cd tests/integration/scenarios/fuzzer/guest && cargo build --target=wasm32-wasip2 --release
	cd examples/tutorial-parser/guest && cargo build --target=wasm32-wasip1 --release

# Run burr install for integration tests
build-int: build build-guests
	@echo "=== Installing burr virtualizer for integration tests ==="
	cd tests/integration/scenarios/telemetry-exfiltration && PATH="$(PWD)/target/release:$$PATH" $(BURR_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/telemetry_logger.wasm"
	cd tests/integration/scenarios/credential-harvester && PATH="$(PWD)/target/release:$$PATH" $(BURR_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/image_processor.wasm"
	cd tests/integration/scenarios/fuzzer && PATH="$(PWD)/target/release:$$PATH" $(BURR_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/adversary_fuzzer.wasm"

# Run burr install for examples
build-ex: build build-guests
	@echo "=== Installing burr virtualizer for examples ==="
	cd examples/tutorial-parser && PATH="$(PWD)/target/release:$$PATH" $(BURR_BIN) install "file://$(PWD)/examples/tutorial-parser/guest/target/wasm32-wasip1/release/parser.wasm"

# Run unit tests
test-unit: build
	@echo "=== Running Unit Tests ==="
	cargo test

# Run end-to-end integration tests
test-int: build-int
	@echo "=== Running Integration Tests ==="
	./tests/integration/run_tests.sh

# Run all tests
test: test-unit test-int

# Clean generated targets and .burr directories
clean:
	@echo "=== Cleaning generated artifacts ==="
	cargo clean
	rm -rf tests/integration/scenarios/telemetry-exfiltration/.burr
	rm -rf tests/integration/scenarios/credential-harvester/.burr
	rm -rf tests/integration/scenarios/fuzzer/.burr
	rm -rf examples/tutorial-parser/.burr

# Install burr locally
install: build
	@echo "=== Installing burr locally ==="
	mkdir -p ~/.cargo/bin
	cp $(BURR_BIN) ~/.cargo/bin/burr
	@echo "Installation complete. Make sure ~/.cargo/bin is in your PATH."
