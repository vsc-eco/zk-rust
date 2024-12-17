use sha2::{Digest, Sha256};

/// Represents a node in a Merkle Tree.
#[derive(Debug, Clone)]
pub struct Node {
    hash: [u8; 32],       // 32-byte hash
    is_leaf: bool,        // Is this node a leaf?
    data: Option<String>, // Data for leaf nodes
}

impl Node {
    /// Create a new leaf node from the given data.
    pub fn new(data: Option<String>) -> Self {
        let is_leaf = data.is_some();
        let hash = if let Some(ref value) = data {
            Self::compute_hash(value.as_bytes())
        } else {
            [0u8; 32]
        };

        Node {
            hash,
            is_leaf,
            data,
        }
    }

    /// Combine two child nodes to create a new parent node.
    pub fn combine(left: &Node, right: &Node) -> Self {
        let combined_hash =
            Self::compute_hash(&[left.hash.as_slice(), right.hash.as_slice()].concat());

        Node {
            hash: combined_hash,
            is_leaf: false,
            data: None,
        }
    }

    /// Hash the contents using SHA256.
    fn compute_hash(input: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().into()
    }

    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }

    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }
}

mod tests {
    #[test]
    fn test_node_creation() {
        let leaf = super::Node::new(Some("test".to_string()));
        assert_eq!(leaf.is_leaf, true);
        assert_eq!(leaf.data, Some("test".to_string()));

        let parent = super::Node::combine(&leaf, &leaf);
        assert_eq!(parent.is_leaf, false);
        assert_eq!(parent.data, None);
    }
}
