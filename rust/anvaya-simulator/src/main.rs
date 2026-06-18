use anvaya_core::qasm::parse_qasm;
use axum::{extract::State, routing::post, Json, Router};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read};
use tower_http::cors::{Any, CorsLayer};

use crate::sim::{simulate_shots, NoiseModel};

mod sim;

#[derive(Parser)]
#[command(name = "anvaya-simulator")]
#[command(about = "Quantum circuit simulator with noise")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Simulate {
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
    },
    Serve {
        #[arg(short, long, default_value = "3001")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Simulate {
            qasm_file,
            shots,
            depolarizing,
            readout_error,
            seed,
        } => {
            let qasm = if let Some(path) = qasm_file {
                fs::read_to_string(&path)?
            } else {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf)?;
                buf
            };
            let circuit = parse_qasm(&qasm)?;
            let noise = NoiseModel {
                depolarizing_prob: depolarizing,
                readout_error,
            };
            let result = simulate_shots(&circuit, shots, &noise, seed)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::Serve { port } => {
            run_server(port).await?;
        }
    }
    Ok(())
}

#[derive(Clone, Default)]
struct AppState;

#[derive(serde::Deserialize)]
struct SimRequest {
    qasm: String,
    #[serde(default = "default_shots")]
    shots: usize,
    depolarizing: Option<f64>,
    readout_error: Option<f64>,
    seed: Option<u64>,
}

fn default_shots() -> usize {
    1024
}

async fn handle_simulate(
    State(_state): State<AppState>,
    Json(req): Json<SimRequest>,
) -> Json<serde_json::Value> {
    let noise = NoiseModel {
        depolarizing_prob: req.depolarizing.unwrap_or(0.001),
        readout_error: req.readout_error.unwrap_or(0.01),
    };
    let result = match parse_qasm(&req.qasm) {
        Ok(circuit) => match simulate_shots(&circuit, req.shots, &noise, req.seed) {
            Ok(r) => serde_json::to_value(r)
                .unwrap_or(serde_json::json!({"error": "serialization failed"})),
            Err(e) => serde_json::json!({"error": format!("Simulation error: {}", e)}),
        },
        Err(e) => serde_json::json!({"error": format!("QASM parse error: {}", e)}),
    };
    Json(result)
}

async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    println!("ANVAYA Simulator Server running on http://{}", addr);

    let app = Router::new()
        .route("/simulate", post(handle_simulate))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(AppState);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
