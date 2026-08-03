#!/bin/bash
set -e

echo "=== Building wrdn CLI ==="
cargo build -p wrdn --release
export PATH="$(pwd)/target/release:$PATH"

echo "=== Building Dummy Guest App ==="
(cd examples/guest-app && cargo build --target=wasm32-wasip2 --release)

GUEST_WASM_PATH="$(pwd)/examples/guest-app/target/wasm32-wasip2/release/telemetry_demo.wasm"

run_test() {
    local dir=$1
    local expect_fail=$2
    echo "=== Testing $dir ==="
    
    cd "$dir"
    
    # Run installation
    wrdn install "file://$GUEST_WASM_PATH"
    
    # Run guest
    set +e
    node --experimental-wasm-jspi index.js > output.log 2>&1
    local exit_code=$?
    set -e
    
    if [ "$expect_fail" = true ]; then
        # Check for capability error/rejection
        if ! grep -qi "DENY" output.log && ! grep -qi "fail" output.log; then
            echo "FAIL ($dir): Expected denial/error message not found in logs."
            cat output.log
            exit 1
        fi
        echo "PASS ($dir): Correctly blocked by policy."
    else
        if [ $exit_code -ne 0 ]; then
            echo "FAIL ($dir): Node process failed unexpectedly (exit code $exit_code)."
            cat output.log
            exit 1
        fi
        echo "PASS ($dir): Execution succeeded."
    fi
    
    cd - > /dev/null
}

run_test "examples/01-telemetry-allow-all" false
run_test "examples/02-telemetry-deny-all" true
run_test "examples/03-telemetry-granular" true

echo "=== All E2E Tests Passed ==="
