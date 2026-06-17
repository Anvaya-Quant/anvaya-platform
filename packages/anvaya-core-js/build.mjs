import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import path from 'path';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const wasmCrateDir = path.resolve(__dirname, '../../rust/anvaya-core-wasm');
const outputDir = path.resolve(__dirname, 'pkg');

console.log('Building ANVAYA Core WASM...');
execSync(
  `wasm-pack build --target web --out-dir ${outputDir} --out-name anvaya_core`,
  { cwd: wasmCrateDir, stdio: 'inherit' }
);
console.log('WASM build complete. Output in', outputDir);
