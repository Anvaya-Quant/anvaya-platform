use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    X,
    Y,
    Z,
    H,
    S,
    T,
    Rx(f64),
    Ry(f64),
    Rz(f64),
    CNOT,
    CZ,
    SWAP,
    Measure,
    Barrier,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gate::X => write!(f, "X"),
            Gate::Y => write!(f, "Y"),
            Gate::Z => write!(f, "Z"),
            Gate::H => write!(f, "H"),
            Gate::S => write!(f, "S"),
            Gate::T => write!(f, "T"),
            Gate::Rx(theta) => write!(f, "Rx({:.3})", theta),
            Gate::Ry(theta) => write!(f, "Ry({:.3})", theta),
            Gate::Rz(theta) => write!(f, "Rz({:.3})", theta),
            Gate::CNOT => write!(f, "CNOT"),
            Gate::CZ => write!(f, "CZ"),
            Gate::SWAP => write!(f, "SWAP"),
            Gate::Measure => write!(f, "M"),
            Gate::Barrier => write!(f, "Barrier"),
        }
    }
}

impl Gate {
    pub fn num_qubits(&self) -> usize {
        match self {
            Gate::X | Gate::Y | Gate::Z | Gate::H | Gate::S | Gate::T
            | Gate::Rx(_) | Gate::Ry(_) | Gate::Rz(_) | Gate::Measure => 1,
            Gate::CNOT | Gate::CZ | Gate::SWAP => 2,
            Gate::Barrier => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_num_qubits() {
        assert_eq!(Gate::X.num_qubits(), 1);
        assert_eq!(Gate::H.num_qubits(), 1);
        assert_eq!(Gate::Rx(0.5).num_qubits(), 1);
        assert_eq!(Gate::CNOT.num_qubits(), 2);
        assert_eq!(Gate::SWAP.num_qubits(), 2);
        assert_eq!(Gate::Measure.num_qubits(), 1);
        assert_eq!(Gate::Barrier.num_qubits(), 0);
    }

    #[test]
    fn test_gate_display() {
        assert_eq!(format!("{}", Gate::X), "X");
        assert_eq!(format!("{}", Gate::H), "H");
        assert_eq!(format!("{}", Gate::Rx(1.2)), "Rx(1.200)");
        assert_eq!(format!("{}", Gate::CNOT), "CNOT");
    }
}
