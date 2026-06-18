import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import path from 'path';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const wasmCrateDir = path.resolve(__dirname, '../../rust/anvaya-pulse-wasm');
const outputDir = path.resolve(__dirname, 'pkg');

console.log('Building ANVAYA Pulse WASM...');
execSync(
  `wasm-pack build --target web --out-dir ${outputDir} --out-name anvaya_pulse`,
  { cwd: wasmCrateDir, stdio: 'inherit' }
);
console.log('Pulse WASM build complete.');
