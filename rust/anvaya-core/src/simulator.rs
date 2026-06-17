use crate::circuit::{Circuit, GateOperation, CircuitError};
use crate::gate::Gate;
use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    CircuitError(CircuitError),
    UnsupportedGate(String),
    InternalError(String),
}

impl From<CircuitError> for SimulationError {
    fn from(e: CircuitError) -> Self {
        SimulationError::CircuitError(e)
    }
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::CircuitError(e) => write!(f, "Circuit error: {}", e),
            SimulationError::UnsupportedGate(g) => write!(f, "Unsupported gate: {}", g),
            SimulationError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}

fn kron(a: &[Vec<Complex64>], b: &[Vec<Complex64>]) -> Vec<Vec<Complex64>> {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_rows = b.len();
    let b_cols = b[0].len();
    let mut result = vec![vec![Complex64::new(0.0, 0.0); a_cols * b_cols]; a_rows * b_rows];
    for i in 0..a_rows {
        for j in 0..a_cols {
            let a_val = a[i][j];
            for k in 0..b_rows {
                for l in 0..b_cols {
                    result[i * b_rows + k][j * b_cols + l] = a_val * b[k][l];
                }
            }
        }
    }
    result
}

fn gate_unitary(gate: &Gate) -> Result<Vec<Vec<Complex64>>, SimulationError> {
    let c = |re, im| Complex64::new(re, im);
    let i = Complex64::i();
    let zero = Complex64::new(0.0, 0.0);
    let one = Complex64::new(1.0, 0.0);
    let sqrt2_inv = FRAC_1_SQRT_2;

    match gate {
        Gate::X => Ok(vec![
            vec![zero, one],
            vec![one,  zero],
        ]),
        Gate::Y => Ok(vec![
            vec![zero, -i],
            vec![i,    zero],
        ]),
        Gate::Z => Ok(vec![
            vec![one,  zero],
            vec![zero, -one],
        ]),
        Gate::H => Ok(vec![
            vec![c(sqrt2_inv, 0.0),  c(sqrt2_inv, 0.0)],
            vec![c(sqrt2_inv, 0.0),  c(-sqrt2_inv, 0.0)],
        ]),
        Gate::S => Ok(vec![
            vec![one, zero],
            vec![zero, c(0.0, 1.0)],
        ]),
        Gate::T => Ok(vec![
            vec![one, zero],
            vec![zero, c(FRAC_1_SQRT_2, FRAC_1_SQRT_2)],
        ]),
        Gate::Rx(theta) => {
            let cos = (*theta / 2.0).cos();
            let sin = (*theta / 2.0).sin();
            Ok(vec![
                vec![c(cos, 0.0),  c(0.0, -sin)],
                vec![c(0.0, -sin), c(cos, 0.0)],
            ])
        }
        Gate::Ry(theta) => {
            let cos = (*theta / 2.0).cos();
            let sin = (*theta / 2.0).sin();
            Ok(vec![
                vec![c(cos, 0.0), c(-sin, 0.0)],
                vec![c(sin, 0.0), c(cos, 0.0)],
            ])
        }
        Gate::Rz(theta) => {
            let half = *theta / 2.0;
            let exp_pos = Complex64::new(half.cos(), half.sin());
            let exp_neg = Complex64::new(half.cos(), -half.sin());
            Ok(vec![
                vec![exp_neg, zero],
                vec![zero,    exp_pos],
            ])
        }
        Gate::CNOT => Ok(vec![
            vec![one,  zero, zero, zero],
            vec![zero, one,  zero, zero],
            vec![zero, zero, zero, one],
            vec![zero, zero, one,  zero],
        ]),
        Gate::CZ => Ok(vec![
            vec![one,  zero, zero, zero],
            vec![zero, one,  zero, zero],
            vec![zero, zero, one,  zero],
            vec![zero, zero, zero, c(-1.0, 0.0)],
        ]),
        Gate::SWAP => Ok(vec![
            vec![one,  zero, zero, zero],
            vec![zero, zero, one,  zero],
            vec![zero, one,  zero, zero],
            vec![zero, zero, zero, one],
        ]),
        Gate::Measure | Gate::Barrier => {
            Ok(vec![vec![one]])
        }
    }
}

fn apply_gate(
    state: &mut Vec<Complex64>,
    gate: &Gate,
    targets: &[usize],
    num_qubits: usize,
) -> Result<(), SimulationError> {
    if gate.num_qubits() == 0 {
        return Ok(());
    }
    if matches!(gate, Gate::Measure) {
        return Ok(());
    }

    let small_u = gate_unitary(gate)?;

    let target_dim = gate.num_qubits();
    if small_u.len() != 1 << target_dim {
        return Err(SimulationError::InternalError("gate unitary dimension mismatch".into()));
    }

    let mut full_u: Vec<Vec<Complex64>> = vec![vec![Complex64::new(1.0, 0.0)]];

    let id2 = vec![
        vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
    ];

    let mut sorted_targets = targets.to_vec();
    sorted_targets.sort();
    let target_set: std::collections::HashSet<usize> = sorted_targets.iter().cloned().collect();

    if target_dim == 1 {
        for q in 0..num_qubits {
            if target_set.contains(&q) {
                full_u = kron(&full_u, &small_u);
            } else {
                full_u = kron(&full_u, &id2);
            }
        }
    } else if target_dim == 2 {
        let n = num_qubits;
        let dim = 1 << n;
        let mut mat = vec![vec![Complex64::new(0.0, 0.0); dim]; dim];
        let t0 = targets[0];
        let t1 = targets[1];
        for i in 0..dim {
            for j in 0..dim {
                let mut match_non_target = true;
                for q in 0..n {
                    if !target_set.contains(&q) {
                        if ((i >> q) & 1) != ((j >> q) & 1) {
                            match_non_target = false;
                            break;
                        }
                    }
                }
                if match_non_target {
                    let i_t0 = (i >> t0) & 1;
                    let i_t1 = (i >> t1) & 1;
                    let j_t0 = (j >> t0) & 1;
                    let j_t1 = (j >> t1) & 1;
                    let row = (i_t1 << 1) | i_t0;
                    let col = (j_t1 << 1) | j_t0;
                    mat[i][j] = small_u[row][col];
                }
            }
        }
        full_u = mat;
    }

    let dim = 1 << num_qubits;
    if full_u.len() != dim || full_u[0].len() != dim {
        return Err(SimulationError::InternalError("full unitary dimension mismatch".into()));
    }
    let mut new_state = vec![Complex64::new(0.0, 0.0); dim];
    for i in 0..dim {
        let mut sum = Complex64::new(0.0, 0.0);
        for j in 0..dim {
            sum += full_u[i][j] * state[j];
        }
        new_state[i] = sum;
    }
    *state = new_state;

    Ok(())
}

pub fn simulate(circuit: &Circuit) -> Result<Vec<Complex64>, SimulationError> {
    let n = circuit.num_qubits;
    let dim = 1usize << n;
    let mut state = vec![Complex64::new(0.0, 0.0); dim];
    state[0] = Complex64::new(1.0, 0.0);

    for GateOperation { gate, targets } in &circuit.operations {
        if targets.iter().any(|&q| q >= n) {
            return Err(SimulationError::CircuitError(CircuitError::QubitOutOfRange {
                qubit: *targets.iter().max().unwrap(),
                max: n - 1,
            }));
        }
        apply_gate(&mut state, gate, targets, n)?;
    }

    Ok(state)
}
