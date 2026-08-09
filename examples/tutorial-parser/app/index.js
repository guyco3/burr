import { parser } from './.wrdn/guyco3_parser/index.js';

async function main() {
    console.log("Starting tutorial parser app...");

    const input = "hello world from WebAssembly component model!";
    console.log(`\nInput text: "${input}"`);

    const wordCount = await parser.countWords(input);
    console.log(`Word count: ${wordCount}`);

    try {
        const uppercase = await parser.parseUppercase(input);
        console.log(`Uppercase output: "${uppercase}"`);
    } catch (e) {
        console.error("\n[!] The component threw an error! The policy engine likely blocked its action.");
        console.error(e.message);
    }
}

main().catch(console.error);
