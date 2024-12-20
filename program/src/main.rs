//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
// use fibonacci_lib::{fibonacci, PublicValuesStruct};
use merkle_tree_lib::{generate_proof_for_solidity, MerkleProofStruct};

pub fn main() {
    // Step 1: Read inputs from zkVM
    let data: Vec<String> = sp1_zkvm::io::read(); // Read list of strings as input
    let index: usize = sp1_zkvm::io::read();     // Read the index of the leaf

    // Step 2: Prepare data as slices for `generate_proof_for_solidity`
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();

    // Step 3: Generate the Merkle proof using the existing function
    let proof_struct = generate_proof_for_solidity(&data_refs, index);

    // Step 4: Encode the proof structure
    let encoded_proof = MerkleProofStruct::abi_encode(&proof_struct);

    // Step 5: Commit the encoded proof to zkVM
    sp1_zkvm::io::commit_slice(&encoded_proof);
}
