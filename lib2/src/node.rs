use crate::hash::Hash;

/// Represents a node in a Merkle Tree.
#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: AsRef<[u8]> + Clone,
{
    hash: Hash,      // Hash of the node
    is_leaf: bool,   // Is this node a leaf?
    data: Option<T>, // Data for leaf nodes
}

impl<T> Node<T>
where
    T: AsRef<[u8]> + Clone,
{
    /// Create a new leaf node from the given data.
    pub fn new(data: Option<T>) -> Self {
        let is_leaf = data.is_some();
        let hash = if let Some(ref value) = data {
            Hash::new(value.as_ref())
        } else {
            Hash::empty()
        };

        Node {
            hash,
            is_leaf,
            data,
        }
    }

    /// Combine two child nodes to create a new parent node.
    pub fn new_from_children(left: &Node<T>, right: &Node<T>) -> Self {
        let combined_hash = Hash::combine(&left.hash, &right.hash);

        Node {
            hash: combined_hash,
            is_leaf: false,
            data: None,
        }
    }

    /// Retrieve the hash of the node.
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    /// Check if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Retrieve the data (if any).
    pub fn data(&self) -> Option<T> {
        self.data.clone()
    }
}


mod tests {
    #[test]
    fn test_node() {
        let data = b"hello world";
        let node = crate::node::Node::new(Some(data));
        assert_eq!(node.is_leaf(), true);
        assert_eq!(node.data(), Some(data));
    }
}
