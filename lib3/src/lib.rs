use alloy_sol_types::sol;
use hex;
use rs_merkle::{algorithms::Sha256 as RsSha256, MerkleProof, MerkleTree};
use sha2::{Digest, Sha256}; // For hashing inputs

sol! {
    struct MerkleProofStruct {
        bytes32[] proof; // Array of sibling hashes
        bytes32 leaf;    // Hash of the leaf node
        bytes32 root;    // Merkle root
    }
}

/// Hashes input data using the `Sha256` hashing algorithm.
fn hash_input(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute the Merkle root for a list of data.
///
/// # Arguments
/// * `data` - A vector of string slices.
///
/// # Returns
/// Merkle root as a hex string.
pub fn compute_merkle_root(data: &[&str]) -> String {
    let leaves: Vec<[u8; 32]> = data.iter().map(|d| hash_input(d.as_bytes())).collect();
    let tree = MerkleTree::<RsSha256>::from_leaves(&leaves);
    hex::encode(tree.root().expect("Root computation failed"))
}

/// Generate a Merkle proof for a specific index in a list of data.
///
/// # Arguments
/// * `data` - A vector of string slices.
/// * `index` - The index of the leaf.
///
/// # Returns
/// A Solidity-compatible struct with proof, leaf, and root.
pub fn generate_proof_for_solidity(data: &[&str], index: usize) -> MerkleProofStruct {
    let leaves: Vec<[u8; 32]> = data.iter().map(|d| hash_input(d.as_bytes())).collect();

    let tree = MerkleTree::<RsSha256>::from_leaves(&leaves);
    let proof = tree.proof(&[index]);
    let root = tree.root().expect("Failed to compute Merkle root");
    let leaf = leaves[index];

    MerkleProofStruct {
        proof: proof
            .proof_hashes()
            .iter()
            .map(|hash| (*hash).into()) // Convert the proof hash into the correct format
            .collect(),
        leaf: leaf.into(), // Convert leaf into the required format
        root: root.into(), // Convert root into the required format
    }
}

/// Verify a Merkle proof.
/// # Arguments
/// * `proof_struct` - A Solidity-compatible proof structure.
/// * `index` - The index of the leaf.
/// * `total_leaves` - Total number of leaves in the tree.
///
/// # Returns
/// `true` if the proof is valid, `false` otherwise.
pub fn verify_proof(proof_struct: &MerkleProofStruct, index: usize, total_leaves: usize) -> bool {
    let proof_hashes: Vec<[u8; 32]> = proof_struct.proof.iter().map(|hash| hash.0).collect();
    let proof = MerkleProof::<RsSha256>::new(proof_hashes);

    // Use the `proof_struct.leaf` directly instead of recomputing
    proof.verify(
        proof_struct.root.0,
        &[index],
        &[proof_struct.leaf.0],
        total_leaves,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_computation() {
        let data = vec!["a", "b", "c", "d"];
        let root = compute_merkle_root(&data);
        println!("Computed Merkle Root: {}", root);

        assert_ne!(root, hex::encode([0u8; 32]), "Root should not be zero");
    }

    #[test]
    fn test_merkle_proof_generation_and_verification() {
        let data = vec!["a", "b", "c", "d"];
        let index = 2;

        // Generate Merkle proof
        let proof_struct = generate_proof_for_solidity(&data, index);

        println!("Merkle Root: {:?}", hex::encode(proof_struct.root));
        println!("Leaf: {:?}", hex::encode(proof_struct.leaf));
        println!(
            "Proof Hashes: {:?}",
            proof_struct
                .proof
                .iter()
                .map(hex::encode)
                .collect::<Vec<_>>()
        );

        // Verify proof
        let is_valid = verify_proof(&proof_struct, index, data.len());
        assert!(is_valid, "Proof should be valid");
    }

    #[test]
    fn test_invalid_proof_verification() {
        let data = vec!["a", "b", "c", "d"];
        let index = 2;

        // Generate valid proof
        let mut proof_struct = generate_proof_for_solidity(&data, index);

        // Tamper with the proof
        proof_struct.leaf = hash_input(b"fake data 123 123").into();

        // Verify tampered proof
        let is_valid = verify_proof(&proof_struct, index, data.len());
        assert!(!is_valid, "Tampered proof should be invalid");
    }
}
