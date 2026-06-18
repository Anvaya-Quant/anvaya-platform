'use client';

import { BaseEdge, EdgeProps, getStraightPath } from '@xyflow/react';

export default function ControlEdge({
  sourceX,
  sourceY,
  targetX,
  targetY,
  markerEnd,
  style,
}: EdgeProps) {
  const [edgePath] = getStraightPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  });

  return (
    <BaseEdge
      path={edgePath}
      markerEnd={markerEnd}
      style={{ stroke: '#60a5fa', strokeWidth: 2, ...style }}
    />
  );
}
