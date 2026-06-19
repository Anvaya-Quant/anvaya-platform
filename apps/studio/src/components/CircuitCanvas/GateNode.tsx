'use client';

import { Handle, Position } from '@xyflow/react';

interface GateNodeProps {
  data: {
    label: string;
    gateType: string;
    qubit: number;
  };
}

const colorMap: Record<string, string> = {
  x: 'bg-red-700 border-red-500',
  y: 'bg-red-700 border-red-500',
  z: 'bg-red-700 border-red-500',
  h: 'bg-blue-700 border-blue-400',
  s: 'bg-indigo-700 border-indigo-400',
  t: 'bg-indigo-700 border-indigo-400',
  cx: 'bg-green-700 border-green-400',
  cnot: 'bg-green-700 border-green-400',
  cz: 'bg-green-700 border-green-400',
  swap: 'bg-green-700 border-green-400',
  rx: 'bg-purple-700 border-purple-400',
  ry: 'bg-purple-700 border-purple-400',
  rz: 'bg-purple-700 border-purple-400',
  measure: 'bg-gray-600 border-gray-400',
  barrier: 'bg-gray-600 border-gray-400',
};

export default function GateNode({ data }: GateNodeProps) {
  const colorClass = colorMap[data.gateType] || 'bg-gray-700 border-gray-500';

  return (
    <div className={`${colorClass} text-white font-mono text-xs px-3 py-1.5 rounded-lg shadow-md border min-w-[44px] text-center transition-colors`} data-testid="gate-node">
      <Handle type="target" position={Position.Top} style={{ visibility: 'hidden' }} />
      {data.label}
      <Handle type="source" position={Position.Bottom} style={{ visibility: 'hidden' }} />
    </div>
  );
}
