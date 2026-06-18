use anvaya_core::circuit::Circuit;
use anvaya_core::gate::Gate;
use anvaya_core::simulator::simulate;
use anvaya_core::optimizer::optimize;
use anvaya_core::qasm::{parse_qasm, to_qasm};

use wasm_bindgen::prelude::*;
use js_sys::Float64Array;

fn state_to_js(state: &[num_complex::Complex64]) -> Float64Array {
    let flat: Vec<f64> = state
        .iter()
        .flat_map(|c| vec![c.re, c.im])
        .collect();
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
        let state = simulate(&self.circuit)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
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
}

#[wasm_bindgen]
pub fn parse_qasm_to_circuit(qasm_str: &str) -> Result<AnvayaCircuit, JsValue> {
    let circuit = parse_qasm(qasm_str).map_err(|e| JsValue::from_str(&e))?;
    Ok(AnvayaCircuit { circuit })
}
