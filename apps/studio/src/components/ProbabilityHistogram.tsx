'use client';

interface ProbabilityHistogramProps {
  probabilities: number[];
  numQubits: number;
}

export default function ProbabilityHistogram({ probabilities, numQubits }: ProbabilityHistogramProps) {
  if (!probabilities || probabilities.length === 0) return null;

  const maxProb = Math.max(...probabilities, 0.01);

  return (
    <div className="flex items-end gap-1 h-32 p-2">
      {probabilities.map((prob, idx) => {
        const heightPercent = (prob / maxProb) * 100;
        const binaryLabel = idx.toString(2).padStart(numQubits, '0');
        return (
          <div key={idx} className="flex flex-col items-center flex-1 min-w-[20px]" data-testid="probability-bar">
            <span className="text-xs text-green-400 mb-1">
              {(prob * 100).toFixed(1)}%
            </span>
            <div
              className="w-full bg-blue-500 rounded-t transition-all duration-300"
              style={{ height: `${Math.max(heightPercent, 1)}%` }}
            />
            <span className="text-xs text-gray-400 mt-1 font-mono">{binaryLabel}</span>
          </div>
        );
      })}
    </div>
  );
}
