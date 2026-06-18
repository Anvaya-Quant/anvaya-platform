use anvaya_core::circuit::Circuit;
use anvaya_core::simulator::simulate;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct NoiseModel {
    pub depolarizing_prob: f64,
    pub readout_error: f64,
}

impl Default for NoiseModel {
    fn default() -> Self {
        NoiseModel {
            depolarizing_prob: 0.001,
            readout_error: 0.01,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SimResult {
    pub num_qubits: usize,
    pub shots: usize,
    pub counts: std::collections::HashMap<String, usize>,
}

pub fn simulate_shots(
    circuit: &Circuit,
    shots: usize,
    noise: &NoiseModel,
    seed: Option<u64>,
) -> Result<SimResult, Box<dyn std::error::Error>> {
    let n = circuit.num_qubits;
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    let ideal_state = simulate(circuit)?;

    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for _ in 0..shots {
        let mut noisy_state = ideal_state.clone();

        if noise.depolarizing_prob > 0.0 {
            apply_depolarizing(&mut noisy_state, noise.depolarizing_prob, &mut rng);
        }

        let probs: Vec<f64> = noisy_state.iter().map(|c| c.norm_sqr()).collect();
        let outcome = sample_from_probabilities(&probs, &mut rng);

        let mut outcome_bits: Vec<bool> = (0..n).map(|i| (outcome >> i) & 1 == 1).collect();
        for bit in outcome_bits.iter_mut() {
            if rng.gen::<f64>() < noise.readout_error {
                *bit = !*bit;
            }
        }

        let final_outcome: usize = outcome_bits
            .iter()
            .enumerate()
            .map(|(i, &b)| if b { 1 << i } else { 0 })
            .sum();
        let bitstring = (0..n)
            .rev()
            .map(|i| {
                if (final_outcome >> i) & 1 == 1 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>();
        *counts.entry(bitstring).or_insert(0) += 1;
    }

    Ok(SimResult {
        num_qubits: n,
        shots,
        counts,
    })
}

fn apply_depolarizing(state: &mut Vec<num_complex::Complex64>, prob: f64, rng: &mut impl Rng) {
    let n = (state.len() as f64).log2() as usize;
    for qubit in 0..n {
        if rng.gen::<f64>() < prob {
            let pauli = rng.gen_range(0..3);
            let mask = 1 << qubit;
            let mut new_state = state.clone();
            match pauli {
                0 => {
                    for (i, val) in state.iter().enumerate() {
                        let flipped = i ^ mask;
                        new_state[flipped] = *val;
                    }
                }
                1 => {
                    for (i, val) in state.iter().enumerate() {
                        if (i & mask) != 0 {
                            new_state[i] = -*val;
                        }
                    }
                }
                2 => {
                    for (i, val) in state.iter().enumerate() {
                        let flipped = i ^ mask;
                        let mut v = *val;
                        if (i & mask) != 0 {
                            v = -v;
                        }
                        new_state[flipped] = v * num_complex::Complex64::i();
                    }
                }
                _ => unreachable!(),
            }
            *state = new_state;
        }
    }
}

fn sample_from_probabilities(probs: &[f64], rng: &mut impl Rng) -> usize {
    let r: f64 = rng.gen();
    let mut cumulative = 0.0;
    for (i, &p) in probs.iter().enumerate() {
        cumulative += p;
        if r < cumulative {
            return i;
        }
    }
    probs.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use anvaya_core::circuit::Circuit;
    use anvaya_core::gate::Gate;

    #[test]
    fn test_simulate_shots_bell() -> Result<(), Box<dyn std::error::Error>> {
        let mut circuit = Circuit::new(2);
        circuit.add_gate(Gate::H, vec![0])?;
        circuit.add_gate(Gate::CNOT, vec![0, 1])?;
        let noise = NoiseModel::default();
        let result = simulate_shots(&circuit, 1000, &noise, Some(42))?;
        let count_00 = *result.counts.get("00").unwrap_or(&0);
        let count_11 = *result.counts.get("11").unwrap_or(&0);
        let total = count_00 + count_11;
        assert!(total > 800);
        Ok(())
    }
}
