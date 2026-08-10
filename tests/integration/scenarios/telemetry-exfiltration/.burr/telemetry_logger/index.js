import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
process.env.WRDN_POLICY_PATH = path.join(__dirname, 'policy.cedar');

export * from './out-guest/guest.js';
