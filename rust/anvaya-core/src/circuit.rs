use crate::gate::Gate;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    pub num_qubits: usize,
    pub operations: Vec<GateOperation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GateOperation {
    pub gate: Gate,
    pub targets: Vec<usize>,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Self {
        Circuit {
            num_qubits,
            operations: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, gate: Gate, targets: Vec<usize>) -> Result<(), CircuitError> {
        if gate.num_qubits() > 0 && targets.len() != gate.num_qubits() {
            return Err(CircuitError::InvalidTargetCount {
                gate: gate.to_string(),
                expected: gate.num_qubits(),
                got: targets.len(),
            });
        }
        for &q in &targets {
            if q >= self.num_qubits {
                return Err(CircuitError::QubitOutOfRange {
                    qubit: q,
                    max: self.num_qubits - 1,
                });
            }
        }
        self.operations.push(GateOperation { gate, targets });
        Ok(())
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Circuit on {} qubits:", self.num_qubits)?;
        for (i, op) in self.operations.iter().enumerate() {
            let targets = op
                .targets
                .iter()
                .map(|q| q.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "  {}: {} [{}]", i, op.gate, targets)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    QubitOutOfRange {
        qubit: usize,
        max: usize,
    },
    InvalidTargetCount {
        gate: String,
        expected: usize,
        got: usize,
    },
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitError::QubitOutOfRange { qubit, max } => {
                write!(f, "Qubit {} out of range (max {})", qubit, max)
            }
            CircuitError::InvalidTargetCount {
                gate,
                expected,
                got,
            } => write!(
                f,
                "Gate {} requires {} target(s), got {}",
                gate, expected, got
            ),
        }
    }
}

impl std::error::Error for CircuitError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::Gate;

    #[test]
    fn test_circuit_creation() {
        let circuit = Circuit::new(3);
        assert_eq!(circuit.num_qubits, 3);
        assert!(circuit.operations.is_empty());
    }

    #[test]
    fn test_add_valid_gate() -> Result<(), CircuitError> {
        let mut circuit = Circuit::new(3);
        circuit.add_gate(Gate::H, vec![0])?;
        circuit.add_gate(Gate::CNOT, vec![0, 1])?;
        assert_eq!(circuit.operations.len(), 2);
        Ok(())
    }

    #[test]
    fn test_invalid_qubit_index() {
        let mut circuit = Circuit::new(2);
        let err = circuit.add_gate(Gate::X, vec![5]).unwrap_err();
        assert!(matches!(
            err,
            CircuitError::QubitOutOfRange { qubit: 5, max: 1 }
        ));
    }

    #[test]
    fn test_invalid_target_count() {
        let mut circuit = Circuit::new(2);
        let err = circuit.add_gate(Gate::CNOT, vec![0]).unwrap_err();
        assert!(matches!(err, CircuitError::InvalidTargetCount { .. }));
    }

    #[test]
    fn test_barrier_accepts_any_targets() {
        let mut circuit = Circuit::new(4);
        assert!(circuit.add_gate(Gate::Barrier, vec![0, 1, 2, 3]).is_ok());
        assert!(circuit.add_gate(Gate::Barrier, vec![]).is_ok());
    }

    #[test]
    fn test_circuit_display() {
        let mut circuit = Circuit::new(2);
        circuit.add_gate(Gate::H, vec![0]).unwrap();
        circuit.add_gate(Gate::CNOT, vec![0, 1]).unwrap();
        let display = format!("{}", circuit);
        assert!(display.contains("Circuit on 2 qubits"));
        assert!(display.contains("H [0]"));
        assert!(display.contains("CNOT [0, 1]"));
    }
}
