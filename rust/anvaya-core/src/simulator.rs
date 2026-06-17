use crate::gate::Gate;
use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    UnsupportedGate(String),
    InternalError(String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
