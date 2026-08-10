import { countWords, parseUppercase } from './.wrdn/guyco3_parser/index.js';

async function main() {
    console.log("Starting tutorial parser app...");

    const input = "hello world from WebAssembly component model!";
    console.log(`\nInput text: "${input}"`);

    const wordCount = await countWords(input);
    console.log(`Word count: ${wordCount}`);

    // We will call the function while passing DEBUG_MODE=1 to Node.js.
    // By default, wrdn's sandbox will block this environment variable,
    // logging an error and hiding it from the guest component.
    const uppercase = await parseUppercase(input);
    console.log(`Uppercase output: "${uppercase}"`);
}

main().catch(console.error);
