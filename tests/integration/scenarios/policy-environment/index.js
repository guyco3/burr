import { guestRunner } from './.burr/policy_env_test/index.js';

async function main() {
    console.log("Starting policy environment test...");
    await guestRunner.execute();
    console.log("Finished executing guest module.");
}

main().catch(err => {
    console.error("Error executing guest:", err);
    process.exit(1);
});
