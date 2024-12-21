#![no_main] // required for zkVM programs as it "wraps" the default main?
sp1_zkvm::entrypoint!(main); // registers `main` as the zkVM entrypoint

use merkle_tree_lib::generate_proof_for_solidity;
use sp1_zkvm::io; // io handling for zkVM

/// zkVM main func for Merkle tree proofs
pub fn main() {
    // reads inputs from the zkVM prover
    let data = io::read::<Vec<String>>(); // read a list of data strings
    let index = io::read::<usize>(); // read the index for which proof is required

    // convert input data into references for processing
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();

    // gen the Merkle proof using the library function (lib3 for now, as I'm transferring it over from lib)
    let proof = generate_proof_for_solidity(&data_refs, index);

    // ABI encode the proof for Solidity compatibility
    let bytes = bincode::serialize(&proof).expect("failed to serialize proof"); // todo: handle gracefully

    // commit the encoded proof to the zkVM
    io::commit_slice(&bytes);
}
