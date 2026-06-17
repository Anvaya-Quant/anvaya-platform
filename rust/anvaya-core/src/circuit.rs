use std::fmt;
use crate::gate::Gate;

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
            let targets = op.targets.iter()
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
    QubitOutOfRange { qubit: usize, max: usize },
    InvalidTargetCount { gate: String, expected: usize, got: usize },
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitError::QubitOutOfRange { qubit, max } =>
                write!(f, "Qubit {} out of range (max {})", qubit, max),
            CircuitError::InvalidTargetCount { gate, expected, got } =>
                write!(f, "Gate {} requires {} target(s), got {}", gate, expected, got),
        }
    }
}

impl std::error::Error for CircuitError {}
