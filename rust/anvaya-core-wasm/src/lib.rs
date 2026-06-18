use anvaya_core::circuit::Circuit;
use anvaya_core::gate::Gate;
use anvaya_core::optimizer::optimize;
use anvaya_core::qasm::{parse_qasm, to_qasm};
use anvaya_core::simulator::simulate;

use js_sys::Float64Array;
use wasm_bindgen::prelude::*;

fn state_to_js(state: &[num_complex::Complex64]) -> Float64Array {
    let flat: Vec<f64> = state.iter().flat_map(|c| vec![c.re, c.im]).collect();
    Float64Array::from(&flat[..])
}

#[wasm_bindgen]
pub struct AnvayaCircuit {
    circuit: Circuit,
}

#[wasm_bindgen]
impl AnvayaCircuit {
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> AnvayaCircuit {
        AnvayaCircuit {
            circuit: Circuit::new(num_qubits),
        }
    }

    pub fn add_gate(
        &mut self,
        gate_name: &str,
        targets: &[usize],
        angle: Option<f64>,
    ) -> Result<(), JsValue> {
        let gate = match gate_name {
            "x" => Gate::X,
            "y" => Gate::Y,
            "z" => Gate::Z,
            "h" => Gate::H,
            "s" => Gate::S,
            "t" => Gate::T,
            "rx" => Gate::Rx(angle.ok_or_else(|| JsValue::from_str("angle required for rx"))?),
            "ry" => Gate::Ry(angle.ok_or_else(|| JsValue::from_str("angle required for ry"))?),
            "rz" => Gate::Rz(angle.ok_or_else(|| JsValue::from_str("angle required for rz"))?),
            "cx" | "cnot" => Gate::CNOT,
            "cz" => Gate::CZ,
            "swap" => Gate::SWAP,
            "measure" => Gate::Measure,
            "barrier" => Gate::Barrier,
            _ => return Err(JsValue::from_str(&format!("unknown gate: {}", gate_name))),
        };
        self.circuit
            .add_gate(gate, targets.to_vec())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn simulate(&self) -> Result<Float64Array, JsValue> {
        let state = simulate(&self.circuit).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(state_to_js(&state))
    }

    pub fn optimize(&mut self) -> usize {
        self.circuit = optimize(&self.circuit);
        self.circuit.operations.len()
    }

    pub fn to_qasm(&self) -> String {
        to_qasm(&self.circuit)
    }

    pub fn parse_qasm(&mut self, qasm_str: &str) -> Result<(), JsValue> {
        let circuit = parse_qasm(qasm_str).map_err(|e| JsValue::from_str(&e))?;
        self.circuit = circuit;
        Ok(())
    }

    pub fn get_gates_json(&self) -> String {
        use anvaya_core::gate::Gate;
        let gates: Vec<serde_json::Value> = self
            .circuit
            .operations
            .iter()
            .map(|op| {
                let (gate_type, angle) = match &op.gate {
                    Gate::X => ("x".to_string(), None),
                    Gate::Y => ("y".to_string(), None),
                    Gate::Z => ("z".to_string(), None),
                    Gate::H => ("h".to_string(), None),
                    Gate::S => ("s".to_string(), None),
                    Gate::T => ("t".to_string(), None),
                    Gate::Rx(theta) => ("rx".to_string(), Some(*theta)),
                    Gate::Ry(theta) => ("ry".to_string(), Some(*theta)),
                    Gate::Rz(theta) => ("rz".to_string(), Some(*theta)),
                    Gate::CNOT => ("cx".to_string(), None),
                    Gate::CZ => ("cz".to_string(), None),
                    Gate::SWAP => ("swap".to_string(), None),
                    Gate::Measure => ("measure".to_string(), None),
                    Gate::Barrier => ("barrier".to_string(), None),
                };
                serde_json::json!({
                    "gate": gate_type,
                    "targets": op.targets,
                    "angle": angle
                })
            })
            .collect();
        serde_json::to_string(&gates).unwrap_or_else(|_| "[]".to_string())
    }
}

#[wasm_bindgen]
pub fn parse_qasm_to_circuit(qasm_str: &str) -> Result<AnvayaCircuit, JsValue> {
    let circuit = parse_qasm(qasm_str).map_err(|e| JsValue::from_str(&e))?;
    Ok(AnvayaCircuit { circuit })
}
