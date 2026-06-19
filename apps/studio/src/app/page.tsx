// SPDX-License-Identifier: Apache-2.0

'use client';

import { useState } from 'react';
import dynamic from 'next/dynamic';
import { useCoreReady } from '@/hooks/useCoreReady';
import { useCircuitStore } from '@/store/circuitStore';
import CircuitCanvas from '@/components/CircuitCanvas/CircuitCanvas';
import GatePalette from '@/components/CircuitCanvas/GatePalette';
import LoadingSpinner from '@/components/LoadingSpinner';
import ProbabilityHistogram from '@/components/ProbabilityHistogram';
import PulseTimeline from '@/components/PulseTimeline';

const BlochSphere = dynamic(() => import('@/components/BlochSphere'), { ssr: false });

export default function Home() {
  const { ready, error } = useCoreReady();
  const {
    numQubits,
    gates,
    stateVector,
    probabilities,
    status,
    setNumQubits,
    simulate,
    clearGates,
    optimize,
    toQasm,
    setProbabilitiesFromCounts,
  } = useCircuitStore();

  const [activeTab, setActiveTab] = useState<'probabilities' | 'pulse'>('probabilities');
  const [pulseQasm, setPulseQasm] = useState('');
  const [paletteOpen, setPaletteOpen] = useState(false);

  const loadPulse = async () => {
    try {
      const qasm = await toQasm();
      setPulseQasm(qasm);
      setActiveTab('pulse');
    } catch (e) {
      console.error(e);
    }
  };

  if (error) return <div className="flex items-center justify-center h-screen bg-anvaya-charcoal text-red-400">Core load error: {error}</div>;
  if (!ready) return <div className="flex items-center justify-center h-screen bg-anvaya-charcoal"><LoadingSpinner message="Loading ANVAYA Core..." /></div>;

  return (
    <main className="flex flex-col h-screen bg-anvaya-charcoal text-gray-100">
      <div className="flex items-center gap-4 p-3 bg-anvaya-dark border-b border-gray-700">
        <h1 className="text-xl font-bold text-anvaya-cobalt mr-4 flex items-center gap-2">
          <span className="w-2 h-2 bg-anvaya-cobalt rounded-full" />
          ANVAYA Studio
        </h1>
        <label className="text-sm">
          Qubits:{' '}
          <input
            type="number"
            min={1}
            max={10}
            value={numQubits}
            onChange={(e) => setNumQubits(Number(e.target.value))}
            className="bg-gray-800 border border-gray-600 rounded px-2 py-1 w-20 text-white"
          />
        </label>
        <button
          onClick={simulate}
          disabled={status === 'simulating'}
          className="bg-green-600 hover:bg-green-500 px-3 py-1 rounded text-sm disabled:opacity-50"
        >
          {status === 'simulating' ? 'Simulating...' : 'Simulate'}
        </button>
        <button
          onClick={async () => {
            try {
              const qasm = await toQasm();
              const response = await fetch('http://localhost:3001/simulate', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ qasm, shots: 1024 }),
              });
              if (!response.ok) throw new Error(`Server error: ${response.status}`);
              const data = await response.json();
              if (data.error) throw new Error(data.error);
              setProbabilitiesFromCounts(data.counts, data.shots);
              setActiveTab('probabilities');
            } catch (err: any) {
              alert('Failed to run on simulator: ' + err.message);
            }
          }}
          disabled={status === 'simulating'}
          className="bg-purple-600 hover:bg-purple-500 px-3 py-1 rounded text-sm disabled:opacity-50"
        >
          Run on Simulator
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
        <button
          onClick={() => setPaletteOpen(!paletteOpen)}
          className="md:hidden bg-gray-700 hover:bg-gray-600 px-3 py-1 rounded text-sm"
        >
          {paletteOpen ? 'Hide Gates' : 'Gates'}
        </button>
      </div>

      <div className="flex flex-col md:flex-row flex-1 overflow-hidden">
        <div className="flex-1 h-1/2 md:h-full relative min-w-0">
          <CircuitCanvas />
        </div>
        <div className={`${paletteOpen ? 'block w-full h-1/2' : 'hidden'} md:block md:w-64 overflow-y-auto bg-anvaya-dark border-l border-gray-700`}>
          <GatePalette />
        </div>
      </div>

      {gates.length > 0 && (
        <div className="bg-anvaya-dark border-t border-gray-700 max-w-full overflow-x-auto">
          <div className="flex border-b border-gray-700">
            <button
              className={`px-4 py-2 text-sm ${
                activeTab === 'probabilities'
                  ? 'bg-gray-800 text-blue-400 border-b-2 border-blue-400'
                  : 'text-gray-500'
              }`}
              onClick={() => setActiveTab('probabilities')}
            >
              Probabilities
            </button>
            <button
              className={`px-4 py-2 text-sm ${
                activeTab === 'pulse'
                  ? 'bg-gray-800 text-blue-400 border-b-2 border-blue-400'
                  : 'text-gray-500'
              }`}
              onClick={loadPulse}
            >
              Pulse
            </button>
          </div>
          <div className="p-4">
            {activeTab === 'probabilities' ? (
              <>
                {numQubits === 1 && stateVector ? (
                  <div>
                    <BlochSphere stateVector={stateVector} />
                    {probabilities && (
                      <>
                        <h2 className="text-sm font-semibold text-gray-300 mt-4 mb-2">Measurement Probabilities</h2>
                        <ProbabilityHistogram probabilities={probabilities} numQubits={numQubits} />
                      </>
                    )}
                    {!probabilities && <p className="text-gray-500">Run simulation to see probabilities.</p>}
                  </div>
                ) : (
                  <>
                    <h2 className="text-sm font-semibold text-gray-300 mb-2">Probabilities</h2>
                    {probabilities ? (
                      <ProbabilityHistogram probabilities={probabilities} numQubits={numQubits} />
                    ) : (
                      <p className="text-gray-500">Run simulation to see probabilities.</p>
                    )}
                  </>
                )}
              </>
            ) : (
              <PulseTimeline qasm={pulseQasm} />
            )}
          </div>
        </div>
      )}
    </main>
  );
}
