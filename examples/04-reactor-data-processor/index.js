import { processor } from './.wrdn/guest/index.js';

console.log("Host [index.js]: Invoking exported processData function...");
const result = processor.processData("Hello Virtualizer!");
console.log("Host [index.js]: Received result:");
console.log(result);
