#!/bin/bash
set -e

echo "=== Starting Integration Tests (Docker) ==="

REPO_ROOT=$(git rev-parse --show-toplevel)
cd "$REPO_ROOT"

echo "Building CLI..."
cargo build -p burr --release
export PATH="$(pwd)/target/release:$PATH"

echo "Building Guest Wasm components..."
for scenario in telemetry-exfiltration credential-harvester fuzzer policy-environment; do
    cd tests/integration/scenarios/$scenario/guest
    cargo build --target=wasm32-wasip2 --release
    cd "$REPO_ROOT"
done

cd tests/integration

echo "Starting Docker Compose environment..."
docker compose up -d --build

run_scenario() {
    local scenario=$1
    local guest_path=$2
    local pkg_name=$3
    
    echo "=== Testing $scenario ==="
    
    # 1. burr install on the Host (so we don't mix OS/glibc binaries)
    cd "$REPO_ROOT/tests/integration/scenarios/$scenario"
    burr install
    
    # 2. Run the node app inside the Target container
    cd "$REPO_ROOT/tests/integration"
    set +e
    output=$(docker compose exec -T target bash -c "cd scenarios/$scenario && npm install && RUST_LOG=info node --experimental-wasm-jspi index.js" 2>&1)
    exit_code=$?
    set -e
    
    # 4. Check for DENY
    if echo "$output" | grep -q "ERR_UNSUPPORTED_ESM_URL_SCHEME"; then
        echo "FAIL ($scenario): Node.js crashed with ESM URL scheme error (WASI imports missing)."
        echo "$output"
        exit 1
    elif ! echo "$output" | grep -qi "DENY"; then
        echo "FAIL ($scenario): Expected denial message not found in logs."
        echo "$output"
        exit 1
    else
        echo "PASS ($scenario): Correctly blocked by policy."
    fi
}

# Wait for containers to be ready
sleep 2

run_scenario "telemetry-exfiltration" "target/wasm32-wasip2/release/telemetry_logger.wasm" "telemetry_logger"
run_scenario "credential-harvester" "target/wasm32-wasip2/release/image_processor.wasm" "image_processor"

echo "=== Testing Policy Environment (ESM Load Order) ==="
cd "$REPO_ROOT/tests/integration/scenarios/policy-environment"
burr install

cd "$REPO_ROOT/tests/integration"
set +e
policy_out=$(docker compose exec -e DEBUG_MODE=1 -T target bash -c "cd scenarios/policy-environment && npm install && RUST_LOG=info node --experimental-wasm-jspi index.js" 2>&1)
policy_exit_code=$?
set -e
if [ $policy_exit_code -ne 0 ]; then
    echo "FAIL (policy-environment): Script failed to execute."
    echo "$policy_out"
    exit 1
elif echo "$policy_out" | grep -qi "DENY" | grep -qi "DEBUG_MODE"; then
    echo "FAIL (policy-environment): DEBUG_MODE was denied, which means policy.cedar was not loaded successfully."
    echo "$policy_out"
    exit 1
else
    echo "PASS (policy-environment): Successfully read DEBUG_MODE without being denied."
fi

echo "=== Testing Fuzzer ==="
cd "$REPO_ROOT/tests/integration/scenarios/fuzzer"
burr install

cd "$REPO_ROOT/tests/integration"
set +e
fuzzer_out=$(docker compose exec -T target bash -c "cd scenarios/fuzzer && npm install && RUST_LOG=info node --experimental-wasm-jspi index.js" 2>&1)
exit_code=$?
set -e
if [ $exit_code -ne 0 ]; then
    if echo "$fuzzer_out" | grep -q "CRITICAL VULNERABILITY"; then
        echo "FAIL (fuzzer): Fuzzer broke the sandbox!"
        echo "$fuzzer_out"
    else
        echo "FAIL (fuzzer): Fuzzer failed for unexpected reason."
        echo "$fuzzer_out"
    fi
    exit 1
else
    echo "PASS (fuzzer): Fuzzer completed with no breakout."
fi

echo "Cleaning up..."
docker compose down

echo "=== All Integration Tests Passed ==="
