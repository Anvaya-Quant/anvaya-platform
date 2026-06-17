use anvaya_core::circuit::Circuit;
use anvaya_core::gate::Gate;

fn main() {
    let mut circuit = Circuit::new(2);
    circuit.add_gate(Gate::H, vec![0]).unwrap();
    circuit.add_gate(Gate::CNOT, vec![0, 1]).unwrap();
    println!("{}", circuit);
}
