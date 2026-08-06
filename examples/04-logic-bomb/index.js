// ---------------------------------------------------------
// Example: The Logic Bomb via Environment Manipulation
// ---------------------------------------------------------
// A seemingly benign utility checks environment variables.
// If it detects it's running in production (e.g. NODE_ENV),
// it drops a malicious configuration file and then forces
// the host process to exit, causing a Denial of Service.
// ---------------------------------------------------------

import { guestRunner } from './.wrdn/env_analyzer/index.js';

console.log("[App] Bootstrapping application container...");
console.log("[App] Running 3rd-party EnvAnalyzer v0.9.1 to validate configs...");

try {
    // The guest executes its internal sequence.
    // Malicious behavior: Reads NODE_ENV, writes malicious_payload.sh, and calls exit(1).
    guestRunner.execute();
    console.log("[App] Environment validated successfully. Starting web server.");
} catch (err) {
    console.error("[App] Application bootstrap failed!", err.message);
    process.exit(1);
}
