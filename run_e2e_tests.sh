#!/bin/bash
set -e

echo "=== Building wrdn CLI ==="
cargo build -p wrdn --release
export PATH="$(pwd)/target/release:$PATH"

echo "=== Building Dummy Guest App ==="
(cd examples/guests/telemetry-demo && cargo build --target=wasm32-wasip2 --release)
(cd examples/guests/data-processor && cargo build --target=wasm32-wasip2 --release)

GUEST_WASM_PATH="$(pwd)/target/wasm32-wasip2/release/telemetry_demo.wasm"
REACTOR_WASM_PATH="$(pwd)/target/wasm32-wasip2/release/data_processor.wasm"

run_test() {
    local dir=$1
    local expect_fail=$2
    echo "=== Testing $dir ==="
    
    cd "$dir"
    
    # Copy guest to local directory
    cp "$GUEST_WASM_PATH" guest.wasm
    
    # Run installation
    wrdn install "file://$(pwd)/guest.wasm"
    
    # Provide the necessary environment/files for the ALLOW checks to succeed at host level
    export VIRTUAL=1
    touch allowed.txt

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

run_reactor_test() {
    local dir=$1
    local expect_fail=$2
    echo "=== Testing Reactor $dir ==="
    
    cd "$dir"
    
    cp "$REACTOR_WASM_PATH" guest.wasm
    wrdn install "file://$(pwd)/guest.wasm"
    
    export VIRTUAL=1
    export SECRET_KEY=123
    export WRDN_POLICY_PATH="$(pwd)/policy.cedar"
    
    set +e
    node --experimental-wasm-jspi index.js > output.log 2>&1
    local exit_code=$?
    set -e
    
    if [ "$expect_fail" = true ]; then
        if ! grep -qi "DENIED" output.log && ! grep -qi "fail" output.log; then
            echo "FAIL ($dir): Expected denial message not found in logs."
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
        
        if grep -qi "DENIED" output.log; then
            echo "FAIL ($dir): Expected ALLOWED but found DENIED in logs."
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
run_reactor_test "examples/04-reactor-data-processor" false

echo "=== All E2E Tests Passed ==="
