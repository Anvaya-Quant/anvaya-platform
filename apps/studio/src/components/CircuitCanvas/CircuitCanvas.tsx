'use client';

import { useCallback, useEffect, useMemo, useRef } from 'react';
import {
  ReactFlow,
  Controls,
  Background,
  BackgroundVariant,
  useNodesState,
  useEdgesState,
  useReactFlow,
  ReactFlowProvider,
  type Node,
  type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import GateNode from './GateNode';
import ControlEdge from './ControlEdge';
import { useCircuitStore, GateOp } from '@/store/circuitStore';

const NODE_TYPES = { gate: GateNode };
const EDGE_TYPES = { control: ControlEdge };

const QUBIT_LABEL_X = 0;
const GATE_START_X = 80;
const GATE_SPACING_X = 100;
const QUBIT_SPACING_Y = 80;

function CircuitCanvasInner() {
  const { numQubits, gates, addGate } = useCircuitStore();
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
  const { fitView } = useReactFlow();

  const { layoutNodes, layoutEdges } = useMemo(() => {
    const nodes: Node[] = [];
    const edges: Edge[] = [];
    const gateCountPerQubit = new Array(numQubits).fill(0);

    for (let q = 0; q < numQubits; q++) {
      nodes.push({
        id: `qubit-${q}`,
        type: 'default',
        data: { label: `q[${q}]` },
        position: { x: QUBIT_LABEL_X, y: q * QUBIT_SPACING_Y + 10 },
        style: {
          background: '#1e293b',
          color: '#94a3b8',
          border: '1px solid #334155',
          width: 60,
          textAlign: 'center',
        },
        draggable: false,
        selectable: false,
      });
    }

    gates.forEach((gateOp: GateOp) => {
      const mainQubit = gateOp.targets[0];
      const x = GATE_START_X + (gateCountPerQubit[mainQubit] || 0) * GATE_SPACING_X;
      const y = mainQubit * QUBIT_SPACING_Y;
      gateCountPerQubit[mainQubit] = (gateCountPerQubit[mainQubit] || 0) + 1;

      nodes.push({
        id: gateOp.id,
        type: 'gate',
        position: { x, y },
        data: {
          label: gateLabel(gateOp.gate, gateOp.angle),
          gateType: gateOp.gate,
          qubit: mainQubit,
        },
        draggable: true,
      });
    });

    const targetMarkerNodes: Node[] = [];
    const finalEdges: Edge[] = [];
    gates.forEach((gateOp) => {
      if (gateOp.targets.length > 1) {
        const sourceNode = nodes.find(n => n.id === gateOp.id);
        const gateX = sourceNode?.position.x || GATE_START_X;
        for (let i = 1; i < gateOp.targets.length; i++) {
          const tq = gateOp.targets[i];
          const markerId = `target-marker-${gateOp.id}-${i}`;
          targetMarkerNodes.push({
            id: markerId,
            type: 'default',
            position: { x: gateX, y: tq * QUBIT_SPACING_Y },
            data: { label: '' },
            style: { width: 4, height: 4, background: 'transparent', border: 'none' },
            draggable: false,
            selectable: false,
          });
          finalEdges.push({
            id: `edge-${gateOp.id}-${i}`,
            source: gateOp.id,
            target: markerId,
            type: 'control',
          });
        }
      }
    });

    return { layoutNodes: [...nodes, ...targetMarkerNodes], layoutEdges: finalEdges };
  }, [gates, numQubits]);

  useEffect(() => {
    setNodes(layoutNodes);
    setEdges(layoutEdges);
  }, [layoutNodes, layoutEdges, setNodes, setEdges]);

  useEffect(() => {
    if (layoutNodes.length > 0) {
      fitView({ padding: 0.3 });
    }
  }, [layoutNodes, fitView]);

  const onDragOver = useCallback((event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'move';
  }, []);

  const onDrop = useCallback(
    (event: React.DragEvent) => {
      event.preventDefault();
      const gateType = event.dataTransfer.getData('application/reactflow-gate');
      const angleStr = event.dataTransfer.getData('application/reactflow-angle');
      const angle = angleStr ? parseFloat(angleStr) : undefined;

      if (!gateType) return;

      const bounds = reactFlowWrapper.current?.getBoundingClientRect();
      if (!bounds) return;
      const y = event.clientY - bounds.top;
      const qubitIndex = Math.round(y / QUBIT_SPACING_Y);
      if (qubitIndex < 0 || qubitIndex >= numQubits) return;

      let targets: number[] = [qubitIndex];
      if (['cx', 'cz', 'cnot', 'swap'].includes(gateType)) {
        if (numQubits < 2) return;
        const target = (qubitIndex + 1) % numQubits;
        targets = [qubitIndex, target];
      }

      addGate({
        id: crypto.randomUUID(),
        gate: gateType,
        targets,
        angle,
      });
    },
    [addGate, numQubits],
  );

  return (
    <div ref={reactFlowWrapper} style={{ width: '100%', height: '100%' }} className="bg-gray-900">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        nodeTypes={NODE_TYPES}
        edgeTypes={EDGE_TYPES}
        onDragOver={onDragOver}
        onDrop={onDrop}
        deleteKeyCode={null}
        multiSelectionKeyCode={null}
        selectionKeyCode={null}
        panOnDrag={[1, 2]}
        selectionOnDrag={false}
        nodesDraggable={true}
        edgesReconnectable={false}
        elementsSelectable={true}
      >
        <Background variant={BackgroundVariant.Dots} gap={20} size={1} color="#334155" />
        <Controls />
      </ReactFlow>
    </div>
  );
}

export default function CircuitCanvas() {
  return (
    <ReactFlowProvider>
      <CircuitCanvasInner />
    </ReactFlowProvider>
  );
}

function gateLabel(gate: string, angle?: number): string {
  switch (gate) {
    case 'x': return 'X';
    case 'y': return 'Y';
    case 'z': return 'Z';
    case 'h': return 'H';
    case 's': return 'S';
    case 't': return 'T';
    case 'rx': return `Rx(${angle?.toFixed(1) || 'θ'})`;
    case 'ry': return `Ry(${angle?.toFixed(1) || 'θ'})`;
    case 'rz': return `Rz(${angle?.toFixed(1) || 'θ'})`;
    case 'cx':
    case 'cnot': return 'CNOT';
    case 'cz': return 'CZ';
    case 'swap': return 'SWAP';
    case 'measure': return 'M';
    case 'barrier': return '||';
    default: return gate;
  }
}
