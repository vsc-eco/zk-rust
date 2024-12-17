use crate::{hash::Hash, node::Node};

#[derive(Debug)]
pub struct MerkleProof {
    proof: Vec<Hash>,
}

impl MerkleProof {
    pub fn new(proof: Vec<Hash>) -> Self {
        MerkleProof { proof }
    }

    // pub fn verify(&self, root: Node<T>) -> Result<bool, Box<dyn std::error::Error>> {
    //     Ok(true) // todo: implement
    // }
}
