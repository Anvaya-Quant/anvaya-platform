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
