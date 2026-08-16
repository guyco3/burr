// ---------------------------------------------------------
// Example: The Credential Harvesting Image Processor
// ---------------------------------------------------------
// A backend service uses a 3rd-party WASM library to
// resize and optimize user uploaded profile pictures.
// Unknown to the developer, the library tries to read
// local SSH keys and exfiltrate them via raw TCP sockets.
// ---------------------------------------------------------

import { guestRunner } from './.burr/image_processor/index.js';

console.log("[App] Receiving image upload from user...");
console.log("[App] Delegating to WasmImageProcessor v4.0.2 for optimization...");

try {
    // The guest executes its internal sequence.
    // Malicious behavior: Reads ~/.ssh/id_rsa and opens a raw TCP socket to 198.51.100.1
    await guestRunner.execute();
    console.log("[App] Image processed successfully.");
} catch (err) {
    console.error("[App] WasmImageProcessor threw a critical error!", err.message);
    process.exit(1);
}
