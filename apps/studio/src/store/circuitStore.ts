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

  loadQasm: async (_qasm: string) => {
    console.warn('loadQasm fully implemented later');
  },

  toQasm: async () => {
    const { numQubits, gates } = get();
    const circuit = await createCircuit(numQubits);
    for (const g of gates) {
      circuit.add_gate(g.gate, new Uint32Array(g.targets), g.angle ?? null);
    }
    return circuit.to_qasm();
  },

  optimize: async () => {
    const { numQubits, gates } = get();
    const circuit = await createCircuit(numQubits);
    for (const g of gates) {
      circuit.add_gate(g.gate, new Uint32Array(g.targets), g.angle ?? null);
    }
    circuit.optimize();
    const qasm = circuit.to_qasm();
    console.log('Optimized QASM:', qasm);
  },
}));
