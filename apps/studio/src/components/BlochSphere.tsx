'use client';

import { useMemo } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Line, Text, Edges } from '@react-three/drei';
import * as THREE from 'three';
import { computeBlochVector } from '@/lib/bloch';

interface BlochSphereProps {
  stateVector: Float64Array | null;
}

export default function BlochSphere({ stateVector }: BlochSphereProps) {
  if (!stateVector || stateVector.length < 4) {
    return (
      <div className="w-full h-64 md:h-80 bg-anvaya-dark rounded-lg border border-gray-700 flex items-center justify-center shadow-lg">
        <span className="text-gray-500 text-sm">Simulate a 1‑qubit circuit to see the Bloch sphere.</span>
      </div>
    );
  }

  const [bx, by, bz] = useMemo(() => computeBlochVector(stateVector), [stateVector]);

  return (
    <div className="w-full h-64 md:h-80 bg-anvaya-dark rounded-lg border border-gray-700 overflow-hidden shadow-lg">
      <Canvas camera={{ position: [2, 2, 2], fov: 45 }}>
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} intensity={1} />
        <OrbitControls enableDamping />
        <BlochContent x={bx} y={by} z={bz} />
      </Canvas>
    </div>
  );
}

function BlochContent({ x, y, z }: { x: number; y: number; z: number }) {
  const arrowLength = Math.sqrt(x * x + y * y + z * z);
  const arrowColor = new THREE.Color(0x60a5fa);

  const axisLength = 1.2;
  const axisPoints = useMemo(() => ({
    x: [new THREE.Vector3(-axisLength, 0, 0), new THREE.Vector3(axisLength, 0, 0)],
    y: [new THREE.Vector3(0, -axisLength, 0), new THREE.Vector3(0, axisLength, 0)],
    z: [new THREE.Vector3(0, 0, -axisLength), new THREE.Vector3(0, 0, axisLength)],
  }), []);

  const quaternion = useMemo(() => {
    const dir = new THREE.Vector3(x, y, z).normalize();
    const zAxis = new THREE.Vector3(0, 0, 1);
    if (dir.dot(zAxis) > 0.9999) return new THREE.Quaternion();
    if (dir.dot(zAxis) < -0.9999) {
      const q = new THREE.Quaternion();
      q.setFromAxisAngle(new THREE.Vector3(1, 0, 0), Math.PI);
      return q;
    }
    return new THREE.Quaternion().setFromUnitVectors(zAxis, dir);
  }, [x, y, z]);

  return (
    <group>
      <mesh>
        <sphereGeometry args={[1, 32, 16]} />
        <meshBasicMaterial color="#1e293b" opacity={0.15} transparent />
        <Edges color="#334155" />
      </mesh>

      <Line points={axisPoints.x} color="red" lineWidth={1} />
      <Line points={axisPoints.y} color="green" lineWidth={1} />
      <Line points={axisPoints.z} color="blue" lineWidth={1} />

      <Text position={[axisLength + 0.1, 0, 0]} fontSize={0.15} color="red">X</Text>
      <Text position={[0, axisLength + 0.1, 0]} fontSize={0.15} color="green">Y</Text>
      <Text position={[0, 0, axisLength + 0.1]} fontSize={0.15} color="blue">Z</Text>

      {arrowLength > 0.001 && (
        <group quaternion={quaternion}>
          <mesh position={[0, 0, arrowLength * 0.45]}>
            <cylinderGeometry args={[0.03, 0.03, arrowLength, 8]} />
            <meshStandardMaterial color={arrowColor} />
          </mesh>
          <mesh position={[0, 0, arrowLength]}>
            <coneGeometry args={[0.08, 0.2, 8]} />
            <meshStandardMaterial color={arrowColor} />
          </mesh>
        </group>
      )}
    </group>
  );
}
