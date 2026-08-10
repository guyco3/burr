import { countWords, parseUppercase } from './.burr/parser/index.js';

async function main() {
    const input = "hello world from WebAssembly component model!";
    const wordCount = await countWords(input);
    const uppercase = await parseUppercase(input);
    
    console.log(`Word count: ${wordCount}`);
    console.log(`Uppercase output: "${uppercase}"`);
}

main().catch(console.error);
