'use client';

import { useState } from 'react';

const SINGLE_QUBIT_GATES = [
  { type: 'x', label: 'X', color: 'bg-red-700 hover:bg-red-600' },
  { type: 'y', label: 'Y', color: 'bg-red-700 hover:bg-red-600' },
  { type: 'z', label: 'Z', color: 'bg-red-700 hover:bg-red-600' },
  { type: 'h', label: 'H', color: 'bg-blue-700 hover:bg-blue-600' },
  { type: 's', label: 'S', color: 'bg-indigo-700 hover:bg-indigo-600' },
  { type: 't', label: 'T', color: 'bg-indigo-700 hover:bg-indigo-600' },
];

const TWO_QUBIT_GATES = [
  { type: 'cx', label: 'CNOT', color: 'bg-green-700 hover:bg-green-600' },
  { type: 'cz', label: 'CZ', color: 'bg-green-700 hover:bg-green-600' },
  { type: 'swap', label: 'SWAP', color: 'bg-green-700 hover:bg-green-600' },
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
    <div className="p-3">
      <h2 className="text-lg font-semibold text-gray-200 mb-3">Gates</h2>

      <div className="space-y-4">
        <div>
          <h3 className="text-xs uppercase tracking-wider text-gray-500 mb-2">Single Qubit</h3>
          <div className="grid grid-cols-2 gap-1">
            {SINGLE_QUBIT_GATES.map((gate) => (
              <div
                key={gate.type}
                className={`${gate.color} text-gray-200 text-sm font-mono px-2 py-1.5 rounded cursor-grab transition-colors flex items-center justify-center`}
                draggable
                onDragStart={(e) => onDragStart(e, gate.type)}
                title={`${gate.label} gate`}
              >
                {gate.label}
              </div>
            ))}
          </div>
        </div>

        <div>
          <h3 className="text-xs uppercase tracking-wider text-gray-500 mb-2">Two Qubit</h3>
          <div className="grid grid-cols-2 gap-1">
            {TWO_QUBIT_GATES.map((gate) => (
              <div
                key={gate.type}
                className={`${gate.color} text-gray-200 text-sm font-mono px-2 py-1.5 rounded cursor-grab transition-colors flex items-center justify-center`}
                draggable
                onDragStart={(e) => onDragStart(e, gate.type)}
                title={`${gate.label} gate`}
              >
                {gate.label}
              </div>
            ))}
          </div>
        </div>

        <div>
          <h3 className="text-xs uppercase tracking-wider text-gray-500 mb-2">Rotations</h3>
          <div className="mb-2">
            <label className="text-xs text-gray-400">Angle (rad):</label>
            <input
              type="number"
              value={rotationAngle}
              onChange={(e) => setRotationAngle(parseFloat(e.target.value) || 0)}
              step={0.1}
              className="bg-gray-700 text-white w-full px-2 py-1 rounded mt-1 text-sm"
            />
          </div>
          <div className="grid grid-cols-3 gap-1">
            {['rx', 'ry', 'rz'].map((rot) => (
              <div
                key={rot}
                className="bg-purple-700 text-gray-200 text-sm font-mono px-2 py-1.5 rounded cursor-grab hover:bg-purple-600 transition-colors flex items-center justify-center"
                draggable
                onDragStart={(e) => onDragStart(e, rot, rotationAngle)}
                title={`${rot.toUpperCase()} rotation`}
              >
                {rot.toUpperCase()}
              </div>
            ))}
          </div>
        </div>

        <div>
          <h3 className="text-xs uppercase tracking-wider text-gray-500 mb-2">Other</h3>
          <div className="grid grid-cols-2 gap-1">
            <div
              className="bg-gray-600 text-gray-200 text-sm font-mono px-2 py-1.5 rounded cursor-grab hover:bg-gray-500 transition-colors flex items-center justify-center"
              draggable
              onDragStart={(e) => onDragStart(e, 'measure')}
              title="Measurement"
            >
              M
            </div>
            <div
              className="bg-gray-600 text-gray-200 text-sm font-mono px-2 py-1.5 rounded cursor-grab hover:bg-gray-500 transition-colors flex items-center justify-center"
              draggable
              onDragStart={(e) => onDragStart(e, 'barrier')}
              title="Barrier"
            >
              ||
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
