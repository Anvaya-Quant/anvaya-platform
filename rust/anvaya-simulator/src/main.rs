use anvaya_core::qasm::parse_qasm;
use clap::Parser;
use std::fs;
use std::io::{self, Read};

mod sim;

#[derive(Parser)]
#[command(name = "anvaya-simulator")]
#[command(about = "Simulate a quantum circuit with noise and return measurement counts")]
struct Args {
    #[arg(short, long)]
    qasm_file: Option<String>,

    #[arg(short = 's', long, default_value = "1024")]
    shots: usize,

    #[arg(long, default_value = "0.001")]
    depolarizing: f64,

    #[arg(long, default_value = "0.01")]
    readout_error: f64,

    #[arg(long)]
    seed: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let qasm = if let Some(path) = args.qasm_file {
        fs::read_to_string(&path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

    let circuit = parse_qasm(&qasm)?;
    let noise = sim::NoiseModel {
        depolarizing_prob: args.depolarizing,
        readout_error: args.readout_error,
    };

    let result = sim::simulate_shots(&circuit, args.shots, &noise, args.seed)?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
