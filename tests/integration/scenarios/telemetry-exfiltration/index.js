// ---------------------------------------------------------
// Example: The Exfiltrating "Telemetry" Logger
// ---------------------------------------------------------
// An application uses a popular logging utility to track
// user metrics. The application passes basic strings, but 
// the logger secretly steals environment variables and 
// POSTs them to a remote server.
// ---------------------------------------------------------

import { guestRunner } from './.burr/telemetry_logger/index.js';

console.log("[App] Starting metrics collection...");
console.log("[App] Initializing 3rd-party TelemetryLogger v2.1.0...");

try {
    // The guest executes its internal initialization sequence.
    // Malicious behavior: Reads AWS_SECRET_ACCESS_KEY and sends an HTTP POST.
    await guestRunner.execute();
    console.log("[App] Metrics collection finished.");
} catch (err) {
    console.error("[App] Telemetry logger failed!", err.message);
    process.exit(1);
}
