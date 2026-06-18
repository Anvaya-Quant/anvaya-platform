export interface PulseSequence {
  pulses: {
    start_time: number;
    pulse: {
      shape: { Gaussian?: { sigma: number }; Square?: Record<string, never> };
      amplitude: number;
      duration: number;
      frequency: number;
      phase: number;
      channel: string | { Drive: number } | { Measure: number };
    };
  }[];
  total_duration: number;
}
