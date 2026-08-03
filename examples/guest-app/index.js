import { processor } from './dist/telemetry_demo.js';

// 1. Helper generator providing healthy mock data records
async function* healthyStream() {
    const batch = [
        { sensorId: "temp-01", value: 22.5, timestamp: BigInt(Date.now()) },
        { sensorId: "temp-01", value: 24.1, timestamp: BigInt(Date.now()) }
    ];
    for (const item of batch) {
        yield item;
    }
}

// 2. Helper generator that triggers a validation error inside Rust
async function* brokenStream() {
    yield { sensorId: "faulty-02", value: -15.0, timestamp: BigInt(Date.now()) };
}

async function runDemo() {
    console.log("--- Test Case 1: Healthy Records ---");
    try {
        const successResult = await processor.analyzeBatch(healthyStream());
        console.log(`Success response: ${successResult}`);
    } catch (e) {
        console.error("Caught error in healthy stream:", e);
    }

    console.log("\n--- Test Case 2: Streaming Error Triggers ---");
    try {
        const errorResult = await processor.analyzeBatch(brokenStream());
        console.log(`Success response: ${errorResult}`);
    } catch (e) {
        if (e && e.tag) {
            console.log(`Caught Error Tag: "${e.tag}"`);
            if (e.tag === 'corrupted-data') {
                console.log(`Reason: ${e.val}`);
            }
        } else {
            console.error("Caught unknown error:", e);
        }
    }
}

runDemo();
