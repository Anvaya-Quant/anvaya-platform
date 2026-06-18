import type { AnvayaCircuit } from '@anvaya/core';

let modulePromise: Promise<typeof import('@anvaya/core')> | null = null;

export async function getCore() {
  if (!modulePromise) {
    if (typeof window === 'undefined') {
      throw new Error('Anvaya Core can only be used in the browser');
    }
    modulePromise = import('@anvaya/core');
  }
  return modulePromise;
}

export async function createCircuit(qubits: number): Promise<AnvayaCircuit> {
  const core = await getCore();
  return new core.AnvayaCircuit(qubits);
}
