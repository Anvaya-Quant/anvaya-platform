import { create } from 'zustand';
import { createCircuit } from '@/lib/anvaya-core';
import type { AnvayaCircuit } from '@anvaya/core';

export interface GateOp {
  id: string;
  gate: string;
  targets: number[];
  angle?: number;
}

interface CircuitState {
  numQubits: number;
  gates: GateOp[];
  stateVector: Float64Array | null;
  probabilities: number[] | null;
  status: 'idle' | 'simulating' | 'error';
  errorMessage: string | null;

  setNumQubits: (n: number) => void;
  addGate: (gate: GateOp) => void;
  removeGate: (id: string) => void;
  clearGates: () => void;
  simulate: () => Promise<void>;
  loadQasm: (qasm: string) => Promise<void>;
  toQasm: () => Promise<string>;
  optimize: () => Promise<void>;
  setProbabilitiesFromCounts: (counts: Record<string, number>, totalShots: number) => void;
}

async function rebuildCircuitFromGates(numQubits: number, gates: GateOp[]): Promise<AnvayaCircuit> {
  const circuit = await createCircuit(numQubits);
  for (const g of gates) {
    circuit.add_gate(g.gate, new Uint32Array(g.targets), g.angle ?? null);
  }
  return circuit;
}

async function extractGatesFromWasm(circuit: AnvayaCircuit): Promise<GateOp[]> {
  const json = circuit.get_gates_json();
  const raw = JSON.parse(json) as Array<{ gate: string; targets: number[]; angle: number | null }>;
  return raw.map((item) => ({
    id: crypto.randomUUID(),
    gate: item.gate,
    targets: item.targets,
    angle: item.angle ?? undefined,
  }));
}

export const useCircuitStore = create<CircuitState>((set, get) => ({
  numQubits: 2,
  gates: [],
  stateVector: null,
  probabilities: null,
  status: 'idle',
  errorMessage: null,

  setNumQubits: (n) => {
    set({ numQubits: n, gates: [], stateVector: null, probabilities: null, status: 'idle' });
  },

  addGate: (gate) => {
    set((state) => ({ gates: [...state.gates, gate] }));
  },

  removeGate: (id) => {
    set((state) => ({ gates: state.gates.filter(g => g.id !== id) }));
  },

  clearGates: () => {
    set({ gates: [], stateVector: null, probabilities: null, status: 'idle' });
  },

  simulate: async () => {
    const { numQubits, gates } = get();
    try {
      set({ status: 'simulating', errorMessage: null });
      const circuit: AnvayaCircuit = await createCircuit(numQubits);
      for (const g of gates) {
        circuit.add_gate(g.gate, new Uint32Array(g.targets), g.angle ?? null);
      }
      const stateVec = circuit.simulate();
      const probs: number[] = [];
      for (let i = 0; i < stateVec.length; i += 2) {
        const re = stateVec[i];
        const im = stateVec[i + 1];
        probs.push(re * re + im * im);
      }
      set({ stateVector: stateVec, probabilities: probs, status: 'idle' });
    } catch (err: any) {
      set({ status: 'error', errorMessage: err.message || String(err) });
    }
  },

  loadQasm: async (qasm: string) => {
    try {
      const { AnvayaCircuit } = await import('@anvaya/core');
      const circuit = new AnvayaCircuit(1);
      circuit.parse_qasm(qasm);
      const gates = await extractGatesFromWasm(circuit);
      const allTargets = gates.flatMap(g => g.targets);
      const maxQubit = allTargets.length > 0 ? Math.max(...allTargets) : 0;
      const numQubits = maxQubit + 1;
      set({ numQubits, gates, stateVector: null, probabilities: null, status: 'idle' });
    } catch (err: any) {
      set({ status: 'error', errorMessage: err.message || String(err) });
    }
  },

  toQasm: async () => {
    const { numQubits, gates } = get();
    const circuit = await rebuildCircuitFromGates(numQubits, gates);
    return circuit.to_qasm();
  },

  optimize: async () => {
    const { numQubits, gates } = get();
    try {
      const circuit = await rebuildCircuitFromGates(numQubits, gates);
      circuit.optimize();
      const optimizedGates = await extractGatesFromWasm(circuit);
      set({ gates: optimizedGates, stateVector: null, probabilities: null, status: 'idle' });
    } catch (err: any) {
      set({ status: 'error', errorMessage: err.message || String(err) });
    }
  },

  setProbabilitiesFromCounts: (counts: Record<string, number>, totalShots: number) => {
    const n = get().numQubits;
    const probs: number[] = new Array(1 << n).fill(0);
    for (const [bitstring, count] of Object.entries(counts)) {
      const idx = parseInt(bitstring, 2);
      if (!isNaN(idx) && idx < probs.length) {
        probs[idx] = count / totalShots;
      }
    }
    set({ probabilities: probs, stateVector: null, status: 'idle' });
  },
}));

if (typeof window !== 'undefined') {
  (window as any).__ANVAYA_STORE__ = useCircuitStore;
}
