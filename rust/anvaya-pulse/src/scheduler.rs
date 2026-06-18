use anvaya_core::circuit::{Circuit, GateOperation};
use anvaya_core::gate::Gate;
use crate::pulse::{
    Channel, Pulse, PulseSequence, PulseShape, ScheduledPulse,
};

#[derive(Debug, Clone)]
pub struct BackendSpec {
    pub single_qubit_gate_time: f64,
    pub two_qubit_gate_time: f64,
    pub measurement_time: f64,
    pub gaussian_sigma: f64,
}

impl Default for BackendSpec {
    fn default() -> Self {
        BackendSpec {
            single_qubit_gate_time: 20.0,
            two_qubit_gate_time: 60.0,
            measurement_time: 100.0,
            gaussian_sigma: 3.0,
        }
    }
}

#[derive(Debug)]
pub enum SchedulerError {
    CircuitError(String),
    UnsupportedGate(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::CircuitError(msg) => write!(f, "Circuit error: {}", msg),
            SchedulerError::UnsupportedGate(g) => write!(f, "Unsupported gate: {}", g),
        }
    }
}

impl std::error::Error for SchedulerError {}

pub fn schedule(circuit: &Circuit, backend: &BackendSpec) -> Result<PulseSequence, SchedulerError> {
    let num_qubits = circuit.num_qubits;
    let mut qubit_time: Vec<f64> = vec![0.0; num_qubits];
    let mut measurement_channel_time = 0.0;
    let mut pulses = Vec::new();

    for op in &circuit.operations {
        match &op.gate {
            Gate::Barrier => continue,
            Gate::Measure => {
                for &q in &op.targets {
                    let start = qubit_time[q].max(measurement_channel_time);
                    let duration = backend.measurement_time;
                    pulses.push(ScheduledPulse {
                        pulse: Pulse::new(
                            PulseShape::Square,
                            1.0,
                            duration,
                            Channel::Measure(q),
                        ),
                        start_time: start,
                    });
                    measurement_channel_time = start + duration;
                    qubit_time[q] = measurement_channel_time;
                }
            }
            Gate::CNOT | Gate::CZ | Gate::SWAP => {
                if op.targets.len() < 2 {
                    return Err(SchedulerError::CircuitError(
                        "two-qubit gate requires at least 2 targets".to_string(),
                    ));
                }
                let q0 = op.targets[0];
                let q1 = op.targets[1];
                let start = qubit_time[q0].max(qubit_time[q1]);
                let duration = backend.two_qubit_gate_time;
                for &q in &[q0, q1] {
                    pulses.push(ScheduledPulse {
                        pulse: Pulse::new(
                            PulseShape::Gaussian { sigma: backend.gaussian_sigma },
                            1.0,
                            duration,
                            Channel::Drive(q),
                        ),
                        start_time: start,
                    });
                }
                qubit_time[q0] = start + duration;
                qubit_time[q1] = start + duration;
            }
            _ => {
                if op.targets.is_empty() {
                    return Err(SchedulerError::CircuitError(
                        "single-qubit gate requires at least 1 target".to_string(),
                    ));
                }
                let q = op.targets[0];
                let start = qubit_time[q];
                let duration = backend.single_qubit_gate_time;
                pulses.push(ScheduledPulse {
                    pulse: Pulse::new(
                        PulseShape::Gaussian { sigma: backend.gaussian_sigma },
                        1.0,
                        duration,
                        Channel::Drive(q),
                    ),
                    start_time: start,
                });
                qubit_time[q] = start + duration;
            }
        }
    }

    let total_duration = qubit_time.into_iter().fold(0.0_f64, f64::max);
    Ok(PulseSequence { pulses, total_duration })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anvaya_core::circuit::Circuit;
    use anvaya_core::gate::Gate;
    use crate::pulse::Channel;

    #[test]
    fn test_single_qubit_gate_schedule() {
        let mut circuit = Circuit::new(1);
        circuit.add_gate(Gate::X, vec![0]).unwrap();
        let backend = BackendSpec::default();
        let seq = schedule(&circuit, &backend).unwrap();
        assert_eq!(seq.pulses.len(), 1);
        assert_eq!(seq.pulses[0].pulse.channel, Channel::Drive(0));
        assert_eq!(seq.pulses[0].start_time, 0.0);
        assert!(seq.total_duration > 0.0);
    }

    #[test]
    fn test_two_qubit_gate_schedule() {
        let mut circuit = Circuit::new(2);
        circuit.add_gate(Gate::CNOT, vec![0, 1]).unwrap();
        let backend = BackendSpec::default();
        let seq = schedule(&circuit, &backend).unwrap();
        assert_eq!(seq.pulses.len(), 2);
        assert_eq!(seq.pulses[0].start_time, seq.pulses[1].start_time);
        assert!(seq.total_duration > 0.0);
    }

    #[test]
    fn test_gates_scheduled_sequentially() {
        let mut circuit = Circuit::new(1);
        circuit.add_gate(Gate::X, vec![0]).unwrap();
        circuit.add_gate(Gate::H, vec![0]).unwrap();
        let backend = BackendSpec::default();
        let seq = schedule(&circuit, &backend).unwrap();
        assert_eq!(seq.pulses.len(), 2);
        let first_end = seq.pulses[0].start_time + seq.pulses[0].pulse.duration;
        assert!(seq.pulses[1].start_time >= first_end);
    }

    #[test]
    fn test_measurement_schedule() {
        let mut circuit = Circuit::new(2);
        circuit.add_gate(Gate::X, vec![0]).unwrap();
        circuit.add_gate(Gate::Measure, vec![0]).unwrap();
        circuit.add_gate(Gate::Measure, vec![1]).unwrap();
        let backend = BackendSpec::default();
        let seq = schedule(&circuit, &backend).unwrap();
        assert_eq!(seq.pulses.len(), 3);
        let measure_pulses: Vec<_> = seq.pulses.iter()
            .filter(|sp| matches!(sp.pulse.channel, Channel::Measure(_)))
            .collect();
        assert_eq!(measure_pulses.len(), 2);
    }
}
