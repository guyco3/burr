import { countWords, parseUppercase } from './.wrdn/guyco3_parser/index.js';

async function main() {
    console.log("Starting tutorial parser app...");

    const input = "hello world from WebAssembly component model!";
    console.log(`\nInput text: "${input}"`);

    const wordCount = await countWords(input);
    console.log(`Word count: ${wordCount}`);

    // We will call the function while passing DEBUG_MODE=1 to Node.js
    // By default, wrdn's sandbox will SILENTLY FILTER this environment variable,
    // so the guest component will never see it, and no debug message will be printed.
    const uppercase = await parseUppercase(input);
    console.log(`Uppercase output: "${uppercase}"`);
    console.log("\n[?] Did you see a [guest] debug message printed above? If not, the wrdn sandbox successfully blocked it!");
}

main().catch(console.error);
