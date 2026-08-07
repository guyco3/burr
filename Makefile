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
	cd examples/guests/01-telemetry-logger && cargo build --target=wasm32-wasip2 --release
	cd examples/guests/02-image-processor && cargo build --target=wasm32-wasip2 --release
	cd examples/guests/03-data-serializer && cargo build --target=wasm32-wasip2 --release
	cd examples/guests/04-env-analyzer && cargo build --target=wasm32-wasip2 --release
	cd examples/guests/adversary-fuzzer && cargo build --target=wasm32-wasip2 --release

# Run wrdn install for each example to generate the .wrdn directories
build-examples: build build-guests
	@echo "=== Installing wrdn virtualizer for examples ==="
	cd examples/01-telemetry-exfiltration && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/telemetry_logger.wasm"
	cd examples/02-credential-harvester && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/image_processor.wasm"
	cd examples/03-silent-backdoor && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/data_serializer.wasm"
	cd examples/04-logic-bomb && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/env_analyzer.wasm"
	cd examples/05-fuzzer && PATH="$(PWD)/target/release:$$PATH" $(WRDN_BIN) install "file://$(PWD)/target/wasm32-wasip2/release/adversary_fuzzer.wasm"

# Run end-to-end tests
test: build build-guests
	@echo "=== Running E2E Tests ==="
	./run_e2e_tests.sh

# Clean generated targets and .wrdn directories
clean:
	@echo "=== Cleaning generated artifacts ==="
	cargo clean
	rm -rf examples/01-telemetry-exfiltration/.wrdn examples/01-telemetry-exfiltration/output.log
	rm -rf examples/02-credential-harvester/.wrdn examples/02-credential-harvester/output.log
	rm -rf examples/03-silent-backdoor/.wrdn examples/03-silent-backdoor/output.log
	rm -rf examples/04-logic-bomb/.wrdn examples/04-logic-bomb/output.log
	rm -rf examples/05-fuzzer/.wrdn examples/05-fuzzer/output.log
