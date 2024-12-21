//! EVM-Compatible Proof for Merkle Tree Program.
//!
//! Commands:
//! Groth16: `RUST_LOG=info cargo run --release --bin evm -- --groth16`
//! PLONK:   `RUST_LOG=info cargo run --release --bin evm -- --plonk`

use clap::Parser;
use merkle_tree_lib::compute_merkle_root;
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use std::path::PathBuf;

/// ELF binary for zkVM
pub const MERKLE_TREE_ELF: &[u8] = include_elf!("merkle-tree-program"); // this is from our program run

/// cli args
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct EVMArgs {
    #[clap(long, value_delimiter = ' ')]
    // needs delimiter to accept space-separated values, else it freaks out
    data: Vec<String>, // merkle tree data

    #[clap(long, default_value = "0")]
    index: usize, // idx of leap

    #[clap(long)]
    groth16: bool, // use groth16

    #[clap(long)]
    plonk: bool, // use plonk
}

/// testing fixture
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1MerkleProofFixture {
    root: String,
    proof: String,
    vkey: String,
}

fn main() {
    // init logging
    sp1_sdk::utils::setup_logger();

    // parse clap args
    let args = EVMArgs::parse();

    // validate flags
    if args.groth16 == args.plonk {
        panic!("Specify exactly one: --groth16 or --plonk");
    }

    // compute Merkle root
    let data_refs: Vec<&str> = args.data.iter().map(String::as_str).collect();
    let root = compute_merkle_root(&data_refs);
    println!("Merkle Root: {:?}", hex::encode(root));

    // setup prover client
    let client = ProverClient::new();

    // setup program
    let (pk, vk) = client.setup(MERKLE_TREE_ELF);

    // prep inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.data); // write data
    stdin.write(&args.index); // write index

    // gen proof
    let proof = if args.groth16 {
        client.prove(&pk, stdin).groth16().run()
    } else {
        client.prove(&pk, stdin).plonk().run()
    }
    .expect("failed to generate proof"); // todo: handle gracefully

    create_proof_fixture(&proof, &vk, args.groth16, root);
}

/// save proof fixture
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    groth16: bool,
    root: [u8; 32],
) {
    let proof_type = if groth16 { "groth16" } else { "plonk" };

    // create fixture structure
    let fixture = SP1MerkleProofFixture {
        root: format!("0x{}", hex::encode(root)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
        vkey: vk.bytes32().to_string(),
    };

    println!("verification Key: {}", fixture.vkey);
    println!("proof: {}", fixture.proof);
    println!("root: {}", fixture.root);

    // save fixture to file
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&path).expect("Failed to create fixture path");
    std::fs::write(
        path.join(format!("{}-fixture.json", proof_type)),
        serde_json::to_string_pretty(&fixture).unwrap(), // todo: handle gracefully
    )
    .expect("failed to write fixture"); // todo: handle gracefully
}
