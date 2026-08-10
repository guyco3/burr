import { fuzzer } from './.burr/guest/index.js';

async function runFuzzer() {
    console.log("--- Running Fuzzer ---");
    try {
        await fuzzer.runFuzzer();
        console.log(`Success`);
    } catch (e) {
        if (e && e.tag) {
            console.error(`Caught Error Tag: "${e.tag}"`, e);
        } else {
            console.error("Caught unknown error:", e);
        }
        process.exit(1);
    }
}

runFuzzer();
