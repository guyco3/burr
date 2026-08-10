.PHONY: all build build-guests build-examples test clean

# Ensure wrdn is built in release mode and in path
WRDN_BIN := $(PWD)/target/release/wrdn

all: build build-guests build-examples

# Build the main wrdn CLI
build:
	@echo "=== Building wrdn CLI ==="
	cargo build -p wrdn --release

# Build all WASM guest components
build-guests:
	@echo "=== Building Supply Chain Guest Apps ==="
	cd tests/integration/scenarios/telemetry-exfiltration/guest && cargo build --target=wasm32-wasip2 --release
	cd tests/integration/scenarios/credential-harvester/guest && cargo build --target=wasm32-wasip2 --release
	cd tests/integration/scenarios/fuzzer/guest && cargo build --target=wasm32-wasip2 --release
	cd examples/tutorial-parser/guest && cargo build --target=wasm32-wasip1 --release

# Run wrdn install for each example to generate the .wrdn directories
build-examples: build build-guests
	@echo "=== Installing wrdn virtualizer for examples ==="
	cd tests/integration/scenarios/telemetry-exfiltration && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/tests/integration/scenarios/telemetry-exfiltration/guest/target/wasm32-wasip2/release/telemetry_logger.wasm"
	cd tests/integration/scenarios/credential-harvester && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/tests/integration/scenarios/credential-harvester/guest/target/wasm32-wasip2/release/image_processor.wasm"
	cd tests/integration/scenarios/fuzzer && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/tests/integration/scenarios/fuzzer/guest/target/wasm32-wasip2/release/adversary_fuzzer.wasm"
	cd examples/tutorial-parser && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/examples/tutorial-parser/guest/target/wasm32-wasip1/release/parser.wasm"

# Run end-to-end tests
test: build
	@echo "=== Running Integration Tests ==="
	./tests/integration/run_tests.sh

# Clean generated targets and .wrdn directories
clean:
	@echo "=== Cleaning generated artifacts ==="
	cargo clean
	rm -rf tests/integration/scenarios/telemetry-exfiltration/.wrdn
	rm -rf tests/integration/scenarios/credential-harvester/.wrdn
	rm -rf tests/integration/scenarios/fuzzer/.wrdn
	rm -rf examples/tutorial-parser/.wrdn

# Install wrdn locally
install: build
	@echo "=== Installing wrdn locally ==="
	mkdir -p ~/.cargo/bin
	cp $(WRDN_BIN) ~/.cargo/bin/wrdn
	@echo "Installation complete. Make sure ~/.cargo/bin is in your PATH."
