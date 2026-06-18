'use client';

import { useState } from 'react';

const GATE_LIST = [
  { type: 'x', label: 'X' },
  { type: 'y', label: 'Y' },
  { type: 'z', label: 'Z' },
  { type: 'h', label: 'H' },
  { type: 's', label: 'S' },
  { type: 't', label: 'T' },
  { type: 'cx', label: 'CNOT' },
  { type: 'cz', label: 'CZ' },
  { type: 'swap', label: 'SWAP' },
];

export default function GatePalette() {
  const [rotationAngle, setRotationAngle] = useState<number>(0);

  const onDragStart = (event: React.DragEvent, gateType: string, angle?: number) => {
    event.dataTransfer.setData('application/reactflow-gate', gateType);
    if (angle !== undefined) {
      event.dataTransfer.setData('application/reactflow-angle', angle.toString());
    }
    event.dataTransfer.effectAllowed = 'move';
  };

  return (
    <div className="w-64 bg-gray-800 border-l border-gray-700 p-4 overflow-y-auto h-full">
      <h2 className="text-lg font-semibold text-gray-200 mb-4">Gates</h2>
      <div className="flex flex-col gap-2">
        {GATE_LIST.map((gate) => (
          <div
            key={gate.type}
            className="bg-gray-700 text-gray-200 px-3 py-2 rounded cursor-grab hover:bg-gray-600 transition-colors flex items-center"
            draggable
            onDragStart={(e) => onDragStart(e, gate.type)}
          >
            <span className="font-mono text-sm">{gate.label}</span>
          </div>
        ))}
        <div className="mt-4">
          <label className="text-xs text-gray-400">Rotation angle (rad):</label>
          <input
            type="number"
            value={rotationAngle}
            onChange={(e) => setRotationAngle(parseFloat(e.target.value) || 0)}
            step={0.1}
            className="bg-gray-700 text-white w-full px-2 py-1 rounded mt-1"
          />
          <div className="flex flex-col gap-1 mt-2">
            {['rx', 'ry', 'rz'].map((rot) => (
              <div
                key={rot}
                className="bg-indigo-700 text-gray-200 px-3 py-2 rounded cursor-grab hover:bg-indigo-600 transition-colors flex items-center"
                draggable
                onDragStart={(e) => onDragStart(e, rot, rotationAngle)}
              >
                <span className="font-mono text-sm">{rot.toUpperCase()}({rotationAngle})</span>
              </div>
            ))}
          </div>
        </div>
        <div className="mt-4">
          <div
            className="bg-red-700 text-gray-200 px-3 py-2 rounded cursor-grab hover:bg-red-600 transition-colors flex items-center"
            draggable
            onDragStart={(e) => onDragStart(e, 'measure')}
          >
            <span className="font-mono text-sm">Measure</span>
          </div>
          <div
            className="bg-gray-600 text-gray-300 px-3 py-2 rounded cursor-grab hover:bg-gray-500 transition-colors flex items-center mt-2"
            draggable
            onDragStart={(e) => onDragStart(e, 'barrier')}
          >
            <span className="font-mono text-sm">Barrier</span>
          </div>
        </div>
      </div>
    </div>
  );
}
