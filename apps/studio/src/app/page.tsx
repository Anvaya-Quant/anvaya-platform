'use client';

import { useCoreReady } from '@/hooks/useCoreReady';
import { useCircuitStore } from '@/store/circuitStore';

export default function Home() {
  const { ready, error } = useCoreReady();
  const {
    numQubits,
    gates,
    probabilities,
    status,
    setNumQubits,
    addGate,
    simulate,
    clearGates,
  } = useCircuitStore();

  if (error) return <div className="text-red-500">Core load error: {error}</div>;
  if (!ready) return <div className="text-gray-400">Loading ANVAYA Core...</div>;

  return (
    <main className="min-h-screen bg-gray-950 text-gray-100 p-8">
      <h1 className="text-3xl font-bold text-blue-400 mb-6">ANVAYA Studio</h1>

      <div className="mb-4">
        <label className="mr-2">Qubits:</label>
        <input
          type="number"
          min={1}
          max={10}
          value={numQubits}
          onChange={(e) => setNumQubits(Number(e.target.value))}
          className="bg-gray-800 border border-gray-700 rounded px-2 py-1 w-20 text-white"
        />
      </div>

      <div className="mb-4 flex gap-2 flex-wrap">
        <button
          onClick={() =>
            addGate({ id: crypto.randomUUID(), gate: 'h', targets: [0] })
          }
          className="bg-blue-600 hover:bg-blue-500 px-3 py-1 rounded"
        >
          Add H(0)
        </button>
        <button
          onClick={() =>
            addGate({ id: crypto.randomUUID(), gate: 'cx', targets: [0, 1] })
          }
          className="bg-blue-600 hover:bg-blue-500 px-3 py-1 rounded"
        >
          Add CX(0,1)
        </button>
        <button
          onClick={simulate}
          disabled={status === 'simulating'}
          className="bg-green-600 hover:bg-green-500 px-3 py-1 rounded disabled:opacity-50"
        >
          {status === 'simulating' ? 'Simulating...' : 'Simulate'}
        </button>
        <button onClick={clearGates} className="bg-gray-700 hover:bg-gray-600 px-3 py-1 rounded">
          Clear
        </button>
      </div>

      <div className="mb-4">
        <h2 className="text-xl mb-2">Circuit gates:</h2>
        <ul className="list-disc list-inside">
          {gates.map((g) => (
            <li key={g.id} className="text-gray-400">
              {g.gate}({g.targets.join(',')})
              {g.angle !== undefined ? ` angle=${g.angle}` : ''}
            </li>
          ))}
        </ul>
        {gates.length === 0 && <p className="text-gray-600">No gates added.</p>}
      </div>

      {probabilities && (
        <div>
          <h2 className="text-xl mb-2">Probabilities:</h2>
          <div className="flex gap-2 flex-wrap">
            {probabilities.map((p, idx) => (
              <div
                key={idx}
                className="bg-gray-800 border border-gray-700 rounded px-3 py-2"
              >
                <span className="text-gray-400">
                  |{idx.toString(2).padStart(numQubits, '0')}⟩:{' '}
                </span>
                <span className="text-green-400">{(p * 100).toFixed(2)}%</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </main>
  );
}
