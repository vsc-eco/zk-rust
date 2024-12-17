use alloy_sol_types::private::Uint;
use alloy_sol_types::sol;

sol! {
    struct MerkleProofPublicValues {
        bytes32 root;        // Merkle root
        uint256 leaf_index;  // Index of the leaf
    }
}

/// Example function to serialize Merkle proof into a Solidity-compatible struct.
pub fn create_solidity_merkle_proof(root: [u8; 32], leaf_index: u32) -> MerkleProofPublicValues {
    MerkleProofPublicValues {
        root: root.into(), // Explicit conversion
        leaf_index: Uint::from(leaf_index),
    }
}
