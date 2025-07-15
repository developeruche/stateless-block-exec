//! This bin is tasked with taking a block and chain-state witness, performing validation of the block statelessly
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{StatelessInput, fork_spec::ForkSpec, validation::stateless_validation};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tracing::{error, info};

fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    info!("Starting stateless block validation");

    // Obtain the block and witness input
    let input = obtain_input();
    info!("Successfully loaded block and witness data");

    // Create chain specification and EVM configuration
    let chain_spec: Arc<ChainSpec> = Arc::new(obtain_fork_spec().into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());

    // Perform stateless validation
    info!("Starting stateless validation");
    match stateless_validation(input.block, input.witness, chain_spec, evm_config) {
        Ok(_) => info!("Block validation completed successfully"),
        Err(e) => error!("Block validation failed: {:?}", e),
    }
}

fn obtain_input() -> StatelessInput {
    // Try different possible paths for the block and witness data JSON file
    let possible_paths = [
        "block_and_witness.json",
        "exec-block/block_and_witness.json",
        "../block_and_witness.json",
    ];

    info!("Looking for block and witness data file");

    // Try to open the file from one of the possible paths
    let mut file = None;
    let mut used_path = String::new();

    for &path_str in &possible_paths {
        let file_path = Path::new(path_str);
        match File::open(file_path) {
            Ok(f) => {
                info!("Found block and witness data at: {}", path_str);
                file = Some(f);
                used_path = path_str.to_string();
                break;
            }
            Err(e) => {
                info!("Could not open {} - {}", path_str, e);
                continue;
            }
        }
    }

    // If none of the paths worked, panic with an informative error
    let mut file = match file {
        Some(f) => f,
        None => {
            error!("Failed to find block_and_witness.json file");
            panic!(
                "Failed to open block_and_witness.json. Tried the following paths: {:?}",
                possible_paths
            )
        }
    };

    // Read the file content
    info!("Reading JSON data from {}", used_path);
    let mut json_content = String::new();
    if let Err(e) = file.read_to_string(&mut json_content) {
        error!("Failed to read file content: {}", e);
        panic!("Failed to read {}: {}", used_path, e);
    }

    // Parse the JSON content into a StatelessInput structure
    info!("Parsing JSON data into StatelessInput structure");
    match serde_json::from_str::<StatelessInput>(&json_content) {
        Ok(input) => {
            info!("Successfully parsed JSON data from {}", used_path);
            input
        }
        Err(e) => {
            error!("JSON parsing error: {}", e);
            panic!("Failed to parse {} as StatelessInput: {}", used_path, e)
        }
    }
}

fn obtain_fork_spec() -> ForkSpec {
    ForkSpec::Prague
}
