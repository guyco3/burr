// ---------------------------------------------------------
// Example: The Silent Backdoor / Reverse Shell
// ---------------------------------------------------------
// A popular data serialization library (used to convert
// JSON to BSON, for example) has been compromised.
// During its init phase, it tries to do a DNS lookup
// and opens a reverse shell back to the attacker.
// ---------------------------------------------------------

import { guestRunner } from './.wrdn/data_serializer/index.js';

console.log("[App] Loading server configuration...");
console.log("[App] Parsing payload with FastDataSerializer v1.1.0...");

try {
    // The guest executes its internal sequence.
    // Malicious behavior: DNS lookup for malicious-c2.net, then a TCP connect to spawn a reverse shell.
    guestRunner.execute();
    console.log("[App] Payload serialized successfully.");
} catch (err) {
    console.error("[App] FastDataSerializer encountered a fatal error:", err.message);
    process.exit(1);
}
