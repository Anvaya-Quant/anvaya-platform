'use client';

import dynamic from 'next/dynamic';
import { useCoreReady } from '@/hooks/useCoreReady';
import { useCircuitStore } from '@/store/circuitStore';
import CircuitCanvas from '@/components/CircuitCanvas/CircuitCanvas';
import GatePalette from '@/components/CircuitCanvas/GatePalette';
import ProbabilityHistogram from '@/components/ProbabilityHistogram';

const BlochSphere = dynamic(() => import('@/components/BlochSphere'), { ssr: false });

export default function Home() {
  const { ready, error } = useCoreReady();
  const {
    numQubits,
    stateVector,
    probabilities,
    status,
    setNumQubits,
    simulate,
    clearGates,
    optimize,
    toQasm,
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
        <button
          onClick={optimize}
          className="bg-indigo-600 hover:bg-indigo-500 px-3 py-1 rounded text-sm"
        >
          Optimize
        </button>
        <button
          onClick={async () => {
            try {
              const qasm = await toQasm();
              const blob = new Blob([qasm], { type: 'text/plain' });
              const url = URL.createObjectURL(blob);
              const a = document.createElement('a');
              a.href = url;
              a.download = 'circuit.qasm';
              a.click();
              URL.revokeObjectURL(url);
            } catch (e) {
              console.error('Export failed', e);
            }
          }}
          className="bg-gray-600 hover:bg-gray-500 px-3 py-1 rounded text-sm"
        >
          Export QASM
        </button>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <div className="flex-1 relative">
          <CircuitCanvas />
        </div>
        <GatePalette />
      </div>

      {((probabilities && probabilities.length > 0) || (stateVector && numQubits === 1)) && (
        <div className="bg-gray-900 border-t border-gray-700 p-4 max-w-full overflow-x-auto">
          {numQubits === 1 && stateVector ? (
            <div>
              <h2 className="text-sm font-semibold text-gray-300 mb-2">Bloch Sphere</h2>
              <BlochSphere stateVector={stateVector} />
              {probabilities && (
                <>
                  <h2 className="text-sm font-semibold text-gray-300 mt-4 mb-2">Measurement Probabilities</h2>
                  <ProbabilityHistogram probabilities={probabilities} numQubits={numQubits} />
                </>
              )}
            </div>
          ) : (
            <>
              <h2 className="text-sm font-semibold text-gray-300 mb-2">Probabilities</h2>
              <ProbabilityHistogram probabilities={probabilities || []} numQubits={numQubits} />
            </>
          )}
        </div>
      )}
    </main>
  );
}
