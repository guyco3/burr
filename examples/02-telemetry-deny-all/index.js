import { telemetry } from './.wrdn/guest/index.js';

async function runDemo() {
    console.log("--- Running Telemetry Demo ---");
    try {
        await telemetry.runDemo();
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

runDemo();

