//! Example for executing or proving the Merkle tree program.

use clap::Parser;
use merkle_tree_lib::compute_merkle_root;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// ELF binary for the zkVM program
pub const MERKLE_TREE_ELF: &[u8] = include_elf!("merkle-tree-program");

/// cli arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, value_delimiter = ' ')]
    // allow it to accept space-separated values (else it freaks out)
    data: Vec<String>, // Input data

    #[clap(long, default_value = "0")]
    index: usize, // idx for proof
}

fn main() {
    // inits logging
    sp1_sdk::utils::setup_logger();

    // parses args (using clap)
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("error: specify either --execute or --prove");
        std::process::exit(1);
    }

    // validate input
    if args.data.is_empty() {
        eprintln!("error: input data cannot be empty");
        std::process::exit(1);
    }

    if args.index >= args.data.len() {
        eprintln!("Error: Index out of bounds for input data.");
        std::process::exit(1);
    }

    // converts input data to references
    let data_refs: Vec<&str> = args.data.iter().map(String::as_str).collect();

    // sets up the prover client
    let client = ProverClient::new();

    // preps input for zkVM
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.data);
    stdin.write(&args.index);

    println!("data: {:?}", args.data);
    println!("idx: {}", args.index);

    if args.execute {
        // executes program
        let (_, report) = client.execute(MERKLE_TREE_ELF, stdin).run().unwrap();
        println!("run successfully");

        // compute & display Merkle root
        let root = compute_merkle_root(&data_refs);
        println!("Merkle root: {:?}", hex::encode(root));

        println!("Cycles: {}", report.total_instruction_count());
    } else {
        // proving setup
        let (pk, vk) = client.setup(MERKLE_TREE_ELF);

        // gen proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof"); // todo: handle gracefully
        println!("proof generated successfully!");

        // verify it
        client
            .verify(&proof, &vk)
            .expect("proof verification failed!"); // todo: handle gracefully
        println!("proof verified!");
    }
}
