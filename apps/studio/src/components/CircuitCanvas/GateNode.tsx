'use client';

import { Handle, Position } from '@xyflow/react';

interface GateNodeProps {
  data: {
    label: string;
    gateType: string;
    qubit: number;
  };
}

export default function GateNode({ data }: GateNodeProps) {
  return (
    <div className="bg-blue-600 text-white font-mono text-xs px-2 py-1 rounded shadow-md border border-blue-400 min-w-[40px] text-center">
      <Handle type="target" position={Position.Top} style={{ visibility: 'hidden' }} />
      {data.label}
      <Handle type="source" position={Position.Bottom} style={{ visibility: 'hidden' }} />
    </div>
  );
}
