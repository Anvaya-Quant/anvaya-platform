'use client';

import { useCoreReady } from '@/hooks/useCoreReady';
import { useCircuitStore } from '@/store/circuitStore';
import CircuitCanvas from '@/components/CircuitCanvas/CircuitCanvas';
import GatePalette from '@/components/CircuitCanvas/GatePalette';

export default function Home() {
  const { ready, error } = useCoreReady();
  const {
    numQubits,
    probabilities,
    status,
    setNumQubits,
    simulate,
    clearGates,
  } = useCircuitStore();

  if (error) return <div className="text-red-500 p-8">Core load error: {error}</div>;
  if (!ready) return <div className="text-gray-400 p-8">Loading ANVAYA Core...</div>;

  return (
    <main className="flex flex-col h-screen bg-gray-950 text-gray-100">
      <div className="flex items-center gap-4 p-3 bg-gray-900 border-b border-gray-700">
        <h1 className="text-xl font-bold text-blue-400 mr-4">ANVAYA Studio</h1>
        <label className="text-sm">
          Qubits:{' '}
          <input
            type="number"
            min={1}
            max={10}
            value={numQubits}
            onChange={(e) => setNumQubits(Number(e.target.value))}
            className="bg-gray-800 border border-gray-600 rounded px-2 py-1 w-16 text-white"
          />
        </label>
        <button
          onClick={simulate}
          disabled={status === 'simulating'}
          className="bg-green-600 hover:bg-green-500 px-3 py-1 rounded text-sm disabled:opacity-50"
        >
          {status === 'simulating' ? 'Simulating...' : 'Simulate'}
        </button>
        <button onClick={clearGates} className="bg-gray-600 hover:bg-gray-500 px-3 py-1 rounded text-sm">
          Clear
        </button>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <div className="flex-1 relative">
          <CircuitCanvas />
        </div>
        <GatePalette />
      </div>

      {probabilities && probabilities.length > 0 && (
        <div className="bg-gray-900 border-t border-gray-700 p-3 max-h-40 overflow-y-auto">
          <h2 className="text-sm font-semibold mb-2">Probabilities</h2>
          <div className="flex flex-wrap gap-2">
            {probabilities.map((p, idx) => (
              <div
                key={idx}
                className="bg-gray-800 border border-gray-700 rounded px-3 py-1 text-sm"
              >
                <span className="text-gray-400">
                  |{idx.toString(2).padStart(numQubits, '0')}⟩:{' '}
                </span>
                <span className="text-green-400">{(p * 100).toFixed(1)}%</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </main>
  );
}
