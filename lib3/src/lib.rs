use alloy_sol_types::sol;
use rs_merkle::{algorithms::Sha256 as RsSha256, MerkleProof, MerkleTree};
use sha2::{Digest, Sha256}; // used for hashing algos

sol! {
    struct MerkleProofStruct {
        bytes32[] proof; // array of sibling hashes
        bytes32 leaf;    // hash(leaf node)
        bytes32 root;    // markle root
    }
}

/// hash the input data using the sha256 alg
fn hash_input(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// compute the Merkle root for a list of data
pub fn compute_merkle_root(data: &[&str]) -> [u8; 32] {
    let leaves: Vec<[u8; 32]> = data.iter().map(|d| hash_input(d.as_bytes())).collect();
    let tree = MerkleTree::<RsSha256>::from_leaves(&leaves);
    tree.root().expect("Root computation failed") // todo: handle gracefully
}

/// gen a Merkle proof for a specific index
pub fn generate_proof_for_solidity(
    data: &[&str],
    index: usize,
) -> (Vec<[u8; 32]>, [u8; 32], [u8; 32]) {
    let leaves: Vec<[u8; 32]> = data.iter().map(|d| hash_input(d.as_bytes())).collect();

    let tree = MerkleTree::<RsSha256>::from_leaves(&leaves);
    let proof = tree.proof(&[index]);
    let root = tree.root().expect("Failed to compute Merkle root"); // todo: handle gracefully
    let leaf = leaves[index];

    (proof.proof_hashes().to_vec(), leaf, root)
}

/// verifies a Merkle proof
pub fn verify_proof(proof: &[Vec<[u8; 32]>; 3], index: usize, total_leaves: usize) -> bool {
    let proof_hashes = &proof[0];
    let root = proof[2][0];
    let leaf = proof[1][0];

    let proof = MerkleProof::<RsSha256>::new(proof_hashes.clone());
    proof.verify(root, &[index], &[leaf], total_leaves)
}

// module-zoned unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_merkle_root_computation() {
        let data = vec!["a", "b", "c", "d"];
        let root = compute_merkle_root(&data);
        println!("computed Merkle Root: {:?}", hex::encode(root));

        assert_ne!(root, [0u8; 32], "root shouldn't be zero");
    }

    #[test]
    fn test_merkle_proof_generation_and_verification() {
        let data = vec!["a", "b", "c", "d"];
        let index = 2;

        let (proof_hashes, leaf, root) = generate_proof_for_solidity(&data, index);

        println!("Merkle root: {:?}", hex::encode(root));
        println!("leaf: {:?}", hex::encode(leaf));
        println!(
            "proof hashes: {:?}",
            proof_hashes.iter().map(hex::encode).collect::<Vec<_>>()
        );

        let proof = [proof_hashes.clone(), vec![leaf], vec![root]];

        let is_valid = verify_proof(&proof, index, data.len());
        assert!(is_valid, "proof should be valid");
    }

    #[test]
    fn test_invalid_proof_verification() {
        let data = vec!["a", "b", "c", "d"];
        let index = 2;

        let (proof_hashes, _, root) = generate_proof_for_solidity(&data, index);

        // tampering!this should make the proof invalid
        let tampered_leaf = hash_input(b"fake data 123 123 hi");

        let proof = [proof_hashes.clone(), vec![tampered_leaf], vec![root]];
        let is_valid = verify_proof(&proof, index, data.len());
        assert!(!is_valid, "tampted proof should be invalid!");
    }
}
