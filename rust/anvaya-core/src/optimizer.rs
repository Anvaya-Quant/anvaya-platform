use crate::circuit::{Circuit, GateOperation};
use crate::gate::Gate;

pub fn optimize(circuit: &Circuit) -> Circuit {
    let mut optimized_ops: Vec<GateOperation> = Vec::new();

    for op in &circuit.operations {
        if let Some(prev) = optimized_ops.last() {
            if prev.targets == op.targets {
                if can_cancel(&prev.gate, &op.gate) {
                    optimized_ops.pop();
                    continue;
                }
                if let Some(merged_gate) = merge_rotations(&prev.gate, &op.gate) {
                    optimized_ops.pop();
                    optimized_ops.push(GateOperation {
                        gate: merged_gate,
                        targets: op.targets.clone(),
                    });
                    continue;
                }
            }
        }
        optimized_ops.push(op.clone());
    }

    Circuit {
        num_qubits: circuit.num_qubits,
        operations: optimized_ops,
    }
}

fn can_cancel(g1: &Gate, g2: &Gate) -> bool {
    if g1 == g2 {
        matches!(
            g1,
            Gate::X | Gate::Y | Gate::Z | Gate::H | Gate::CNOT | Gate::CZ | Gate::SWAP
        )
    } else {
        false
    }
}

fn merge_rotations(g1: &Gate, g2: &Gate) -> Option<Gate> {
    match (g1, g2) {
        (Gate::Rx(a), Gate::Rx(b)) => Some(Gate::Rx(a + b)),
        (Gate::Ry(a), Gate::Ry(b)) => Some(Gate::Ry(a + b)),
        (Gate::Rz(a), Gate::Rz(b)) => Some(Gate::Rz(a + b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::gate::Gate;
    use crate::simulator::simulate;
    use approx::assert_abs_diff_eq;
    use num_complex::Complex64;

    fn assert_state_similar(a: &[Complex64], b: &[Complex64]) {
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b) {
            assert_abs_diff_eq!(x.re, y.re, epsilon = 1e-10);
            assert_abs_diff_eq!(x.im, y.im, epsilon = 1e-10);
        }
    }

    fn build_circuit(ops: Vec<GateOperation>, num_qubits: usize) -> Circuit {
        Circuit {
            num_qubits,
            operations: ops,
        }
    }

    #[test]
    fn test_cancel_xx() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::X,
                    targets: vec![0],
                },
                GateOperation {
                    gate: Gate::X,
                    targets: vec![0],
                },
            ],
            1,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 0);
        let orig_state = simulate(&c).unwrap();
        let opt_state = simulate(&opt).unwrap();
        assert_state_similar(&orig_state, &opt_state);
    }

    #[test]
    fn test_cancel_hh() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::H,
                    targets: vec![0],
                },
                GateOperation {
                    gate: Gate::H,
                    targets: vec![0],
                },
            ],
            1,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 0);
        let orig = simulate(&c).unwrap();
        let opt_sim = simulate(&opt).unwrap();
        assert_state_similar(&orig, &opt_sim);
    }

    #[test]
    fn test_cancel_cnot() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::CNOT,
                    targets: vec![0, 1],
                },
                GateOperation {
                    gate: Gate::CNOT,
                    targets: vec![0, 1],
                },
            ],
            2,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 0);
        let orig = simulate(&c).unwrap();
        let opt_sim = simulate(&opt).unwrap();
        assert_state_similar(&orig, &opt_sim);
    }

    #[test]
    fn test_merge_rx() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::Rx(0.5),
                    targets: vec![0],
                },
                GateOperation {
                    gate: Gate::Rx(0.7),
                    targets: vec![0],
                },
            ],
            1,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 1);
        if let Gate::Rx(angle) = opt.operations[0].gate {
            assert_abs_diff_eq!(angle, 1.2, epsilon = 1e-10);
        } else {
            panic!("expected Rx gate");
        }
        let orig = simulate(&c).unwrap();
        let opt_sim = simulate(&opt).unwrap();
        assert_state_similar(&orig, &opt_sim);
    }

    #[test]
    fn test_merge_ry() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::Ry(1.0),
                    targets: vec![1],
                },
                GateOperation {
                    gate: Gate::Ry(-0.5),
                    targets: vec![1],
                },
            ],
            2,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 1);
        if let Gate::Ry(angle) = opt.operations[0].gate {
            assert_abs_diff_eq!(angle, 0.5, epsilon = 1e-10);
        } else {
            panic!("expected Ry gate");
        }
        let orig = simulate(&c).unwrap();
        let opt_sim = simulate(&opt).unwrap();
        assert_state_similar(&orig, &opt_sim);
    }

    #[test]
    fn test_non_cancelling_gates_remain() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::X,
                    targets: vec![0],
                },
                GateOperation {
                    gate: Gate::Z,
                    targets: vec![0],
                },
            ],
            1,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 2);
        let orig = simulate(&c).unwrap();
        let opt_sim = simulate(&opt).unwrap();
        assert_state_similar(&orig, &opt_sim);
    }

    #[test]
    fn test_mixed_qubits_no_unwanted_merge() {
        let c = build_circuit(
            vec![
                GateOperation {
                    gate: Gate::H,
                    targets: vec![0],
                },
                GateOperation {
                    gate: Gate::H,
                    targets: vec![1],
                },
            ],
            2,
        );
        let opt = optimize(&c);
        assert_eq!(opt.operations.len(), 2);
        let orig = simulate(&c).unwrap();
        let opt_sim = simulate(&opt).unwrap();
        assert_state_similar(&orig, &opt_sim);
    }
}
