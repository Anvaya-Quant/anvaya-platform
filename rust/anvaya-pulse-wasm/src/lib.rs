// SPDX-License-Identifier: AGPL-3.0-or-later

use anvaya_core::qasm::parse_qasm;
use anvaya_pulse::scheduler::{schedule, BackendSpec};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn schedule_from_qasm(qasm_str: &str) -> Result<String, JsValue> {
    let circuit =
        parse_qasm(qasm_str).map_err(|e| JsValue::from_str(&format!("QASM parse error: {}", e)))?;
    let backend = BackendSpec::default();
    let seq = schedule(&circuit, &backend).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&seq)
        .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
}
