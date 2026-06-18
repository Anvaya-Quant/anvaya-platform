'use client';

import { useEffect, useState } from 'react';
import type { PulseSequence } from '@/lib/pulse-types';

interface PulseBar {
  channel: string;
  start_time: number;
  duration: number;
  amplitude: number;
}

interface SeqDisplay {
  bars: PulseBar[];
  totalDuration: number;
  channels: string[];
}

function formatChannel(ch: string | { Drive: number } | { Measure: number }): string {
  if (typeof ch === 'string') return ch;
  if ('Drive' in ch) return `Drive(${ch.Drive})`;
  if ('Measure' in ch) return `Measure(${ch.Measure})`;
  return String(ch);
}

async function fetchPulseSequence(qasm: string): Promise<SeqDisplay | null> {
  try {
    const pulseModule = await import('@anvaya/pulse');
    await (pulseModule as any).default();
    const json: string = pulseModule.schedule_from_qasm(qasm);
    const seq: PulseSequence = JSON.parse(json);
    const channelSet = new Set<string>();
    const bars: PulseBar[] = seq.pulses.map((p) => {
      const ch = formatChannel(p.pulse.channel);
      channelSet.add(ch);
      return {
        channel: ch,
        start_time: p.start_time,
        duration: p.pulse.duration,
        amplitude: p.pulse.amplitude,
      };
    });
    const channels = Array.from(channelSet).sort();
    return { bars, totalDuration: seq.total_duration, channels };
  } catch (e) {
    console.error('Pulse scheduling error:', e);
    return null;
  }
}

export default function PulseTimeline({ qasm }: { qasm: string }) {
  const [data, setData] = useState<SeqDisplay | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!qasm) return;
    setLoading(true);
    fetchPulseSequence(qasm).then((d) => {
      setData(d);
      setLoading(false);
    });
  }, [qasm]);

  if (!qasm) return <div className="text-gray-500">No circuit to display.</div>;
  if (loading) return <div className="text-gray-400">Loading pulse sequence...</div>;
  if (!data) return <div className="text-red-400">Failed to generate pulse sequence.</div>;

  const { bars, totalDuration, channels } = data;
  const maxTime = Math.max(totalDuration, 1);
  const timeToX = (t: number) => (t / maxTime) * 100;

  return (
    <div className="min-w-0">
      <h3 className="text-sm font-semibold text-gray-300 mb-2">Pulse Sequence</h3>
      <div className="relative overflow-x-auto">
        <div className="flex flex-col gap-1">
          {channels.map((ch) => (
            <div key={ch} className="flex items-center h-6">
              <span className="w-24 text-xs text-gray-500 shrink-0">{ch}</span>
              <div className="relative flex-1 h-4 bg-gray-800 rounded-sm min-w-[200px]">
                {bars
                  .filter((b) => b.channel === ch)
                  .map((b, i) => (
                    <div
                      key={i}
                      className="absolute top-0 h-full bg-blue-600 opacity-80 rounded-sm"
                      style={{
                        left: `${timeToX(b.start_time)}%`,
                        width: `${Math.max((b.duration / maxTime) * 100, 0.5)}%`,
                      }}
                      title={`${b.amplitude} \u00d7 ${b.duration}ns`}
                    />
                  ))}
              </div>
            </div>
          ))}
        </div>
        <div className="flex text-xs text-gray-600 mt-1">
          <span className="w-24 shrink-0" />
          <div className="flex-1 relative h-4 min-w-[200px]">
            {[0, 0.25, 0.5, 0.75, 1].map((frac) => (
              <span key={frac} className="absolute" style={{ left: `${frac * 100}%` }}>
                {(maxTime * frac).toFixed(0)}ns
              </span>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
